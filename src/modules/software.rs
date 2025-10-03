use crate::utils::break_long_text;
use regex::Regex;
use std::fs;
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
    let shell_path = std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    let shell_name = shell_path.split('/').last().unwrap_or("unknown");
    let version = if shell_name == "bash" {
        Command::new(shell_name)
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let re = Regex::new(r"version (\d+\.\d+\.\d+(?:\(\d+\))?-\w+)").ok()?;
                    re.captures(&stdout).map(|caps| caps[1].to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "unknown".to_string())
    } else if shell_name == "zsh" {
        Command::new(shell_name)
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let re = Regex::new(r"zsh (\d+\.\d+\.\d+)").ok()?;
                    re.captures(&stdout).map(|caps| caps[1].to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "unknown".to_string())
    } else if shell_name == "fish" {
        Command::new(shell_name)
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let re = Regex::new(r"version (\d+\.\d+\.\d+)").ok()?;
                    re.captures(&stdout).map(|caps| caps[1].to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "unknown".to_string())
    } else {
        // Generic fallback
        Command::new(shell_name)
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    // Look for version pattern in output
                    let re = Regex::new(r"(\d+\.\d+\.\d+)").ok()?;
                    re.captures(&stdout).map(|caps| caps[1].to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "unknown".to_string())
    };

    format!("{} {}", shell_name, version)
}

// fn get_os_info_from_os_release() -> (String, String) {
//     let os_release_paths = ["/etc/os-release", "/usr/lib/os-release"];
//
//     for path in &os_release_paths {
//         if let Ok(contents) = fs::read_to_string(path) {
//             let mut pretty_name = None;
//             let mut version = None;
//
//             for line in contents.lines() {
//                 if line.starts_with("PRETTY_NAME=") {
//                     pretty_name = Some(
//                         line.trim_start_matches("PRETTY_NAME=")
//                             .trim_matches('"')
//                             .to_string(),
//                     );
//                 } else if line.starts_with("VERSION=") {
//                     version = Some(
//                         line.trim_start_matches("VERSION=")
//                             .trim_matches('"')
//                             .to_string(),
//                     );
//                 }
//             }
//
//             match (&pretty_name, &version) {
//                 (Some(name), Some(ver)) => return (name.clone(), ver.clone()),
//                 (Some(name), None) => return (name.clone(), "Unknown".to_string()),
//                 _ => continue,
//             }
//         }
//     }
//
//     ("Unknown".to_string(), "Unknown".to_string())
// }

fn get_package_count() -> String {
    let mut package_counts = Vec::new();

    // dpkg (Debian/Ubuntu)
    let dpkg_count = Command::new("sh")
        .arg("-c")
        .arg("dpkg --list | grep '^ii' | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if dpkg_count != "0" {
        package_counts.push(format!("{} (dpkg)", dpkg_count));
    }

    // apt (Debian/Ubuntu)
    let apt_count = Command::new("sh")
        .arg("-c")
        .arg("apt list --installed 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if apt_count != "0" {
        package_counts.push(format!("{} (apt)", apt_count));
    }

    // pacman (Arch Linux)
    let pacman_count = Command::new("sh")
        .arg("-c")
        .arg("pacman -Q 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if pacman_count != "0" {
        package_counts.push(format!("{} (pacman)", pacman_count));
    }

    // yay (Arch Linux AUR)
    let yay_count = Command::new("sh")
        .arg("-c")
        .arg("yay -Q 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if yay_count != "0" {
        package_counts.push(format!("{} (yay)", yay_count));
    }

    // paru (Arch Linux AUR)
    let paru_count = Command::new("sh")
        .arg("-c")
        .arg("paru -Q 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if paru_count != "0" {
        package_counts.push(format!("{} (paru)", paru_count));
    }

    // rpm (Red Hat/Fedora)
    let rpm_count = Command::new("sh")
        .arg("-c")
        .arg("rpm -qa 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if rpm_count != "0" {
        package_counts.push(format!("{} (rpm)", rpm_count));
    }

    // dnf (Fedora)
    let dnf_count = Command::new("sh")
        .arg("-c")
        .arg("dnf list installed 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if dnf_count != "0" {
        package_counts.push(format!("{} (dnf)", dnf_count));
    }

    // yum (Red Hat/CentOS)
    let yum_count = Command::new("sh")
        .arg("-c")
        .arg("yum list installed 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if yum_count != "0" {
        package_counts.push(format!("{} (yum)", yum_count));
    }

    // zypper (openSUSE)
    let zypper_count = Command::new("sh")
        .arg("-c")
        .arg("zypper search --installed-only 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if zypper_count != "0" {
        package_counts.push(format!("{} (zypper)", zypper_count));
    }

    // emerge (Gentoo)
    let emerge_count = Command::new("sh")
        .arg("-c")
        .arg("qlist -I 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if emerge_count != "0" {
        package_counts.push(format!("{} (emerge)", emerge_count));
    }

    // nix (NixOS)
    let nix_count = Command::new("sh")
        .arg("-c")
        .arg("nix-store -q --requisites /run/current-system/sw 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if nix_count != "0" {
        package_counts.push(format!("{} (nix)", nix_count));
    }

    // brew (macOS/Linux)
    let brew_count = Command::new("sh")
        .arg("-c")
        .arg("brew list 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if brew_count != "0" {
        package_counts.push(format!("{} (brew)", brew_count));
    }

    // flatpak
    let flatpak_count = Command::new("sh")
        .arg("-c")
        .arg("flatpak list --app 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if flatpak_count != "0" {
        package_counts.push(format!("{} (flatpak)", flatpak_count));
    }

    // snap
    let snap_count = Command::new("sh")
        .arg("-c")
        .arg("snap list 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if snap_count != "0" {
        package_counts.push(format!("{} (snap)", snap_count));
    }

    // cargo (Rust)
    let cargo_count = Command::new("sh")
        .arg("-c")
        .arg("cargo install --list 2>/dev/null | grep '^[a-zA-Z]' | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if cargo_count != "0" {
        package_counts.push(format!("{} (cargo)", cargo_count));
    }

    // pip (Python)
    let pip_count = Command::new("sh")
        .arg("-c")
        .arg("pip list 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if pip_count != "0" {
        package_counts.push(format!("{} (pip)", pip_count));
    }

    // npm (Node.js)
    let npm_count = Command::new("sh")
        .arg("-c")
        .arg("npm list -g --depth=0 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if npm_count != "0" {
        package_counts.push(format!("{} (npm)", npm_count));
    }

    // portage (Gentoo)
    let portage_count = Command::new("sh")
        .arg("-c")
        .arg("ls /var/db/pkg/*/*/PF 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if portage_count != "0" {
        package_counts.push(format!("{} (portage)", portage_count));
    }

    // apk (Alpine Linux)
    let apk_count = Command::new("sh")
        .arg("-c")
        .arg("apk info 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if apk_count != "0" {
        package_counts.push(format!("{} (apk)", apk_count));
    }

    // xbps (Void Linux)
    let xbps_count = Command::new("sh")
        .arg("-c")
        .arg("xbps-query -l 2>/dev/null | grep '^ii' | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if xbps_count != "0" {
        package_counts.push(format!("{} (xbps)", xbps_count));
    }

    // pkg (FreeBSD)
    let pkg_count = Command::new("sh")
        .arg("-c")
        .arg("pkg info 2>/dev/null | wc -l")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if pkg_count != "0" {
        package_counts.push(format!("{} (pkg)", pkg_count));
    }

    // Return formatted string
    if package_counts.is_empty() {
        "No packages found".to_string()
    } else {
        package_counts.join(", ")
    }
}
