use display_info::DisplayInfo;
use std::env;
use std::fs;
use std::process::Command;
use sysinfo::System;

// pub struct InfoLine {
//     pub label: String,
//     pub value: String,
// }

pub fn get_display_info() -> Vec<(String, String)> {
    // pub fn get_display_info() -> Vec<InfoLine> {
    let mut info = Vec::new();

    let wm_de = get_wm_de();
    info.push(("WM/DE".to_string(), wm_de));
    // info.push(InfoLine {
    //     label: "WM/DE".to_string(),
    //     value: wm_de,
    // });

    let swap = get_swap_info();
    info.push(("Swap".to_string(), swap));
    // info.push(InfoLine {
    //     label: "SWAP".to_string(),
    //     value: swap,
    // });

    let architecture = get_architecture();
    info.push(("Arch".to_string(), architecture));
    // info.push(InfoLine {
    //     label: "Arch".to_string(),
    //     value: architecture,
    // });

    let cursor = get_cursor_theme();
    info.push(("Cursor".to_string(), cursor));
    // info.push(InfoLine {
    //     label: "Cursor".to_string(),
    //     value: cursor,
    // });

    let resolution = get_screen_resolution();
    info.push(("Resolution".to_string(), resolution));
    // info.push(InfoLine {
    //     label: "Resolution".to_string(),
    //     value: resolution,
    // });

    info
}

fn get_screen_resolution() -> String {
    if let Ok(display_infos) = DisplayInfo::all() {
        if !display_infos.is_empty() {
            // primary display
            if let Some(primary_display) = display_infos.iter().find(|d| d.is_primary) {
                return format!("{}x{}", primary_display.width, primary_display.height);
            }
            // If no primary, use the first display
            if let Some(first_display) = display_infos.first() {
                return format!("{}x{}", first_display.width, first_display.height);
            }
        }
    }

    // Fallback: using xrandr command
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("xrandr --current | grep '*' | head -1 | awk '{print $1}'")
        .output()
    {
        if output.status.success() {
            let resolution = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !resolution.is_empty() {
                return resolution;
            }
        }
    }

    "Unknown".to_string()
}

fn get_wm_de() -> String {
    if let Ok(wayland_display) = env::var("WAYLAND_DISPLAY") {
        if !wayland_display.is_empty() {
            if env::var("SWAYSOCK").is_ok() {
                let version = crate::utils::get_version("sway", &["--version"])
                    .unwrap_or_else(|| "unknown".to_string());
                return format!("Sway {} (Wayland)", version);
            }
            if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
                let version = crate::utils::get_version("hyprctl", &["version"])
                    .unwrap_or_else(|| "unknown".to_string());
                return format!("Hyprland {} (Wayland)", version);
            }
            if let Ok(xdg_current_desktop) = env::var("XDG_CURRENT_DESKTOP") {
                return format!("{} (X11)", xdg_current_desktop);
            }
            return "Wayland".to_string();
        }
    }

    if let Ok(display) = env::var("DISPLAY") {
        if !display.is_empty() {
            if let Ok(xdg_current_desktop) = env::var("XDG_CURRENT_DESKTOP") {
                return format!("{} (X11)", xdg_current_desktop);
            }
        }
    }

    "Unknown".to_string()
}

fn get_swap_info() -> String {
    let mut system = System::new_all();
    system.refresh_all();
    let total_swap = system.total_swap() as f64 / (1024.0 * 1024.0);
    let used_swap = system.used_swap() as f64 / (1024.0 * 1024.0);

    format!("{:.1} MiB / {:.1} MiB", used_swap, total_swap)
}

fn get_architecture() -> String {
    Command::new("uname")
        .arg("-m")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn get_cursor_theme() -> String {
    if let Ok(wayland_display) = env::var("WAYLAND_DISPLAY") {
        if !wayland_display.is_empty() {
            // Sway cursor theme detection
            if env::var("SWAYSOCK").is_ok() {
                if let Ok(home_dir) = env::var("HOME") {
                    let sway_config_path = format!("{}/.config/sway/config", home_dir);
                    if let Ok(contents) = fs::read_to_string(&sway_config_path) {
                        for line in contents.lines() {
                            let trimmed = line.trim();
                            if trimmed.starts_with("seat") && trimmed.contains("cursor_theme") {
                                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                                for part in parts {
                                    if part.starts_with("cursor_theme") {
                                        let theme_parts: Vec<&str> = part.splitn(2, ' ').collect();
                                        if theme_parts.len() > 1 {
                                            let theme = theme_parts[1].trim().trim_matches('"');
                                            if !theme.is_empty() {
                                                return theme.to_string();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Hyprland cursor theme detection
            if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
                if let Ok(home_dir) = env::var("HOME") {
                    let hypr_config_path = format!("{}/.config/hypr/hyprland.conf", home_dir);
                    if let Ok(contents) = fs::read_to_string(&hypr_config_path) {
                        for line in contents.lines() {
                            if line.trim().starts_with("cursor:") {
                                let parts: Vec<&str> = line.splitn(2, ':').collect();
                                if parts.len() > 1 {
                                    let theme = parts[1].trim().trim_matches(',').trim();
                                    if !theme.is_empty() {
                                        return theme.to_string();
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Generic Wayland cursor theme
            if let Ok(theme) = env::var("XCURSOR_THEME") {
                return theme;
            }
        }
    }

    // X11 cursor detection (existing code)
    if let Ok(home_dir) = env::var("HOME") {
        let gtk_settings_path = format!("{}/.config/gtk-3.0/settings.ini", home_dir);
        if let Ok(contents) = fs::read_to_string(&gtk_settings_path) {
            for line in contents.lines() {
                if line.trim().starts_with("gtk-cursor-theme-name=") {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() > 1 {
                        let theme = parts[1]
                            .trim()
                            .trim_matches('"')
                            .trim_matches('\'')
                            .to_string();
                        if !theme.is_empty() {
                            return theme;
                        }
                    }
                }
            }
        }

        let xresources_path = format!("{}/.Xresources", home_dir);
        if let Ok(contents) = fs::read_to_string(&xresources_path) {
            for line in contents.lines() {
                if line.trim().starts_with("Xcursor.theme:") {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() > 1 {
                        let theme = parts[1]
                            .trim()
                            .trim_matches('"')
                            .trim_matches('\'')
                            .to_string();
                        if !theme.is_empty() {
                            return theme;
                        }
                    }
                }
            }
        }
    }

    if let Ok(theme) = env::var("XCURSOR_THEME") {
        return theme;
    }

    // Final fallback: check current cursor using command
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("gsettings get org.gnome.desktop.interface cursor-theme 2>/dev/null || echo Unknown")
        .output()
    {
        if output.status.success() {
            let theme = String::from_utf8_lossy(&output.stdout)
                .trim()
                .trim_matches('\'')
                .to_string();
            if theme != "Unknown" {
                return theme;
            }
        }
    }

    "Unknown".to_string()
}
