use regex::Regex;
use std::env;
use std::fs;
use std::process::Command;

pub fn get_terminal_info() -> Vec<(String, String)> {
    let mut info = Vec::new();

    let terminal = get_terminal_info_internal();
    info.push(("Terminal".to_string(), terminal));

    let font = get_terminal_font();
    info.push(("Font".to_string(), font));

    info
}

fn get_terminal_info_internal() -> String {
    let term = env::var("TERM").unwrap_or_else(|_| "unknown".to_string());

    if term.contains("kitty") {
        let version = get_terminal_version("kitty").unwrap_or_else(|| "unknown".to_string());
        return format!("kitty {} ({})", version, term);
    }

    if let Ok(term_program) = env::var("TERM_PROGRAM") {
        let term_program_version =
            env::var("TERM_PROGRAM_VERSION").unwrap_or_else(|_| "unknown".to_string());
        if term_program_version != "unknown" {
            return format!("{} {} ({})", term_program, term_program_version, term);
        } else {
            let version =
                get_terminal_version(&term_program).unwrap_or_else(|| "unknown".to_string());
            if version != "unknown" {
                return format!("{} {} ({})", term_program, version, term);
            } else {
                return format!("{} ({})", term_program, term);
            }
        }
    }

    if let Ok(kitty_pid) = env::var("KITTY_PID") {
        if !kitty_pid.is_empty() {
            let version = get_terminal_version("kitty").unwrap_or_else(|| "unknown".to_string());
            return format!("kitty {} ({})", version, term);
        }
    }

    if let Ok(alacritty_log) = env::var("ALACRITTY_LOG") {
        if !alacritty_log.is_empty() {
            let version =
                get_terminal_version("alacritty").unwrap_or_else(|| "unknown".to_string());
            return format!("alacritty {} ({})", version, term);
        }
    }

    if let Ok(gnome_terminal_id) = env::var("GNOME_TERMINAL_SCREEN") {
        if !gnome_terminal_id.is_empty() {
            let version =
                get_terminal_version("gnome-terminal").unwrap_or_else(|| "unknown".to_string());
            return format!("gnome-terminal {} ({})", version, term);
        }
    }

    term
}

fn get_terminal_version(terminal: &str) -> Option<String> {
    let output = Command::new(terminal)
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })?;

    let version_re = Regex::new(r"\b(\d+\.\d+(?:\.\d+)?)\b").unwrap();
    if let Some(captures) = version_re.captures(&output) {
        return Some(captures[1].to_string());
    }

    let parts: Vec<&str> = output.trim().split_whitespace().collect();
    if parts.len() >= 2 && parts[1].chars().any(|c| c.is_ascii_digit()) {
        return Some(parts[1].to_string());
    }

    for part in parts {
        if part.chars().any(|c| c.is_ascii_digit()) && part.contains('.') {
            return Some(part.to_string());
        }
    }

    None
}

fn get_terminal_font() -> String {
    let term = env::var("TERM").unwrap_or_else(|_| "unknown".to_string());

    if let Ok(term_program) = env::var("TERM_PROGRAM") {
        match term_program.to_lowercase().as_str() {
            "iterm2" => get_iterm2_font().unwrap_or_else(|| "Unknown".to_string()),
            "apple_terminal" => get_apple_terminal_font().unwrap_or_else(|| "Unknown".to_string()),
            _ => detect_font_from_config().unwrap_or_else(|| "Unknown".to_string()),
        }
    } else if term.contains("kitty") {
        get_kitty_font().unwrap_or_else(|| "Unknown".to_string())
    } else if term.contains("alacritty") {
        get_alacritty_font().unwrap_or_else(|| "Unknown".to_string())
    } else {
        detect_font_from_config().unwrap_or_else(|| "Unknown".to_string())
    }
}

fn get_iterm2_font() -> Option<String> {
    let home_dir = env::var("HOME").ok()?;
    let plist_path = format!(
        "{}/Library/Preferences/com.googlecode.iterm2.plist",
        home_dir
    );

    if let Ok(contents) = fs::read_to_string(plist_path) {
        let font_re = Regex::new(r#"<key>Normal Font</key>\s*<string>(.*?)</string>"#).ok()?;
        if let Some(caps) = font_re.captures(&contents) {
            return Some(caps[1].to_string());
        }
    }
    None
}

fn get_apple_terminal_font() -> Option<String> {
    let home_dir = env::var("HOME").ok()?;
    let plist_path = format!("{}/Library/Preferences/com.apple.Terminal.plist", home_dir);

    if let Ok(contents) = fs::read_to_string(plist_path) {
        let font_re = Regex::new(r#"<key>Font</key>\s*<string>(.*?)</string>"#).ok()?;
        if let Some(caps) = font_re.captures(&contents) {
            return Some(caps[1].to_string());
        }
    }
    None
}

fn get_kitty_font() -> Option<String> {
    let home_dir = env::var("HOME").ok()?;
    let config_paths = [
        format!("{}/.config/kitty/kitty.conf", home_dir),
        format!("{}/.kitty.conf", home_dir),
    ];

    for path in &config_paths {
        if let Ok(contents) = fs::read_to_string(path) {
            for line in contents.lines() {
                if line.trim().starts_with("font_family") {
                    let parts: Vec<&str> = line.splitn(2, ' ').collect();
                    if parts.len() > 1 {
                        return Some(parts[1].trim().trim_matches('"').to_string());
                    }
                }
            }
        }
    }
    None
}

fn get_alacritty_font() -> Option<String> {
    let home_dir = env::var("HOME").ok()?;
    let config_paths = [
        format!("{}/.config/alacritty/alacritty.toml", home_dir),
        format!("{}/.config/alacritty/alacritty.yml", home_dir),
        format!("{}/.alacritty.toml", home_dir),
        format!("{}/.alacritty.yml", home_dir),
    ];

    for path in &config_paths {
        if let Ok(contents) = fs::read_to_string(path) {
            if path.ends_with(".toml") {
                for line in contents.lines() {
                    if line.trim().starts_with("family =") {
                        let re = Regex::new(r#""([^"]+)""#).ok()?;
                        if let Some(caps) = re.captures(line) {
                            return Some(caps[1].to_string());
                        }
                    }
                }
            } else {
                for line in contents.lines() {
                    if line.trim().starts_with("family:") {
                        let parts: Vec<&str> = line.splitn(2, ':').collect();
                        if parts.len() > 1 {
                            return Some(parts[1].trim().replace('"', "").to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

fn detect_font_from_config() -> Option<String> {
    let home_dir = env::var("HOME").ok()?;
    let config_paths = [
        format!("{}/.Xresources", home_dir),
        format!("{}/.Xdefaults", home_dir),
        format!("{}/.xresources", home_dir),
        format!("{}/.xdefaults", home_dir),
    ];

    for path in &config_paths {
        if let Ok(contents) = fs::read_to_string(path) {
            for line in contents.lines() {
                if line.trim().starts_with("xterm*font:") || line.trim().starts_with("*.font:") {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() > 1 {
                        return Some(parts[1].trim().trim_matches('"').to_string());
                    }
                }
            }
        }
    }

    if let Ok(output) = Command::new("xrdb").arg("-query").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.starts_with("*.font:") || line.starts_with("xterm*font:") {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() > 1 {
                        return Some(parts[1].trim().trim_matches('"').to_string());
                    }
                }
            }
        }
    }

    None
}
