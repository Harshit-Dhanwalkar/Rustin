use crate::utils::break_long_text;
use std::process::Command;

pub fn get_software_info() -> Vec<(String, String)> {
    let mut info = Vec::new();

    // Shell info
    let shell = get_shell_info();
    info.push(("Shell".to_string(), shell));

    // Packages info
    let packages = get_package_count();
    let packages_lines = break_long_text(&packages, 35);
    for (i, line) in packages_lines.iter().enumerate() {
        let label = if i == 0 {
            "Packages".to_string()
        } else {
            "".to_string()
        };
        info.push((label, line.clone()));
    }

    info
}

fn get_shell_info() -> String {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    let shell_name = shell.split('/').last().unwrap_or("unknown");
    let version_output = Command::new(shell_name)
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string());

    let version = if shell_name == "bash" {
        version_output
            .split_whitespace()
            .nth(3)
            .unwrap_or("unknown")
            .to_string()
    } else if shell_name == "zsh" {
        version_output
            .split_whitespace()
            .nth(1)
            .unwrap_or("unknown")
            .to_string()
    } else if shell_name == "fish" {
        version_output
            .split_whitespace()
            .nth(2)
            .unwrap_or("unknown")
            .to_string()
    } else {
        version_output
            .split_whitespace()
            .find(|word| word.chars().any(|c| c.is_ascii_digit()))
            .unwrap_or("unknown")
            .to_string()
    };

    format!("{} {}", shell_name, version)
}

fn get_package_count() -> String {
    let dpkg_count = Command::new("sh")
        .arg("-c")
        .arg("dpkg --list | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());

    let apt_count = Command::new("sh")
        .arg("-c")
        .arg("apt list --installed 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());

    let brew = Command::new("sh")
        .arg("-c")
        .arg("brew list | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());

    let flatpak_count = Command::new("sh")
        .arg("-c")
        .arg("flatpak list --app | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());

    // .arg("pacman -Q | wc -l")
    // .arg("rpm -qa | wc -l")
    format!(
        "{} (dpkg), {} (apt), {} (brew), {} (flatpak)",
        dpkg_count, apt_count, flatpak_count, brew
    )
}
