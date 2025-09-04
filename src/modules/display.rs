use std::env;
use std::fs;
use std::process::Command;
use sysinfo::System;

pub fn get_display_info() -> Vec<(String, String)> {
    let mut info = Vec::new();

    let wm_de = get_wm_de();
    info.push(("WM/DE".to_string(), wm_de));

    let swap = get_swap_info();
    info.push(("Swap".to_string(), swap));

    let architecture = get_architecture();
    info.push(("Arch".to_string(), architecture));

    let cursor = get_cursor_theme();
    info.push(("Cursor".to_string(), cursor));

    info
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
            return "Wayland".to_string();
        }
    }

    if let Ok(display) = env::var("DISPLAY") {
        if !display.is_empty() {
            if let Ok(wm) = Command::new("wmctrl").arg("-m").output() {
                if let Some(name) = String::from_utf8_lossy(&wm.stdout)
                    .lines()
                    .find(|line| line.starts_with("Name:"))
                {
                    let wm_name = name.replace("Name:", "").trim().to_string();
                    let version = match wm_name.to_lowercase().as_str() {
                        "gnome" => crate::utils::get_version("gnome-shell", &["--version"]),
                        "xfce" | "xfwm4" => crate::utils::get_version("xfwm4", &["--version"]),
                        "kde" | "kwin_x11" => crate::utils::get_version("kwin_x11", &["--version"]),
                        _ => None,
                    }
                    .unwrap_or_else(|| "unknown".to_string());
                    return format!("{} {} (X11)", wm_name, version);
                }
            }
            return "X11".to_string();
        }
    }

    "Unknown".to_string()
}

fn get_swap_info() -> String {
    let system = System::new_all();
    let total_swap = system.total_swap() as f64 / (1024.0 * 1024.0 * 1024.0);
    let used_swap = system.used_swap() as f64 / (1024.0 * 1024.0 * 1024.0);

    if total_swap > 0.0 {
        let percentage = (used_swap / total_swap) * 100.0;
        format!(
            "{:.1}G / {:.1}G ({:.0}%)",
            used_swap, total_swap, percentage
        )
    } else {
        "No swap".to_string()
    }
}

fn get_architecture() -> String {
    let arch = Command::new("uname")
        .arg("-m")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| whoami::arch().to_string());

    arch
}

fn get_cursor_theme() -> String {
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/".to_string());

    if let Ok(output) = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.interface")
        .arg("cursor-theme")
        .output()
    {
        if output.status.success() {
            let cursor_theme = String::from_utf8_lossy(&output.stdout)
                .trim()
                .trim_matches('\'')
                .to_string();
            if !cursor_theme.is_empty() && cursor_theme != "''" {
                return cursor_theme;
            }
        }
    }

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

    if let Ok(theme) = env::var("XCURSOR_THEME") {
        return theme;
    }

    "Unknown".to_string()
}
