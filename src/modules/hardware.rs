use crate::utils::break_long_text;
use std::process::Command;
use sysinfo::{Disks, System};

pub fn get_hardware_info() -> Vec<(String, String)> {
    let mut system = System::new_all();
    system.refresh_all();
    let disks = Disks::new_with_refreshed_list();

    let mut info = Vec::new();

    // CPU info
    let cpu_brand = system
        .cpus()
        .first()
        .map_or_else(|| "Unknown".to_string(), |cpu| cpu.brand().to_string());
    let cpu_lines = break_long_text(&cpu_brand, 35);
    for (i, line) in cpu_lines.iter().enumerate() {
        let label = if i == 0 {
            "CPU".to_string()
        } else {
            "".to_string()
        };
        info.push((label, line.clone()));
    }

    // GPU info
    let gpu = get_gpu_info();
    let gpu_lines = break_long_text(&gpu, 35);
    for (i, line) in gpu_lines.iter().enumerate() {
        let label = if i == 0 {
            "GPU".to_string()
        } else {
            "".to_string()
        };
        info.push((label, line.clone()));
    }

    // Memory info
    let total_memory = system.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let used_memory = system.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let memory_percentage = (used_memory / total_memory) * 100.0;
    info.push((
        "Memory".to_string(),
        format!(
            "{:.1}G / {:.1}G ({:.0}%)",
            used_memory, total_memory, memory_percentage
        ),
    ));

    // Disk info
    for disk in disks.list() {
        let total_space = disk.total_space() as f64 / (1024.0 * 1024.0 * 1024.0);
        let available_space = disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0);
        let used_space = total_space - available_space;
        let percentage = (used_space / total_space) * 100.0;
        info.push((
            "Disk".to_string(),
            format!(
                "{:.1}G / {:.1}G ({:.0}%)",
                used_space, total_space, percentage
            ),
        ));
    }

    // Battery info
    let battery = get_battery_info();
    let battery_lines = break_long_text(&battery, 35);
    for (i, line) in battery_lines.iter().enumerate() {
        let label = if i == 0 {
            "Battery".to_string()
        } else {
            "".to_string()
        };
        info.push((label, line.clone()));
    }

    info
}

fn get_gpu_info() -> String {
    Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i 'vga\\|3d\\|display' | head -n1 | sed -E 's/.*: (.*) \\(rev.*\\)/\\1/'")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn get_battery_info() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("upower -i $(upower -e | grep 'BAT')")
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut native_path = "Unknown".to_string();
        let mut model = "Unknown".to_string();
        let mut percentage = "Unknown".to_string();
        let mut state = "Unknown".to_string();

        for line in stdout.lines() {
            if line.contains("native-path:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    native_path = parts[1].trim_end_matches("...").to_string();
                }
            } else if line.contains("model:") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() > 1 {
                    model = parts[1].trim().to_string();
                }
            } else if line.contains("percentage:") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() > 1 {
                    percentage = parts[1].trim().to_string();
                }
            } else if line.contains("state:") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() > 1 {
                    state = if parts[1].trim() == "charging" {
                        "Yes".to_string()
                    } else {
                        "No".to_string()
                    };
                }
            }
        }

        if native_path != "Unknown" {
            format!(
                "{} {} - {} (Char:{})",
                native_path, model, percentage, state
            )
        } else {
            "No battery found".to_string()
        }
    } else {
        "No battery found".to_string()
    }
}
