use sysinfo::System;
use whoami;

pub fn get_system_info() -> Vec<(String, String)> {
    let mut system = System::new_all();
    system.refresh_all();

    let username = whoami::username();
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

    let uptime_seconds = System::uptime();
    let uptime_hours = uptime_seconds / 3600;
    let uptime_minutes = (uptime_seconds % 3600) / 60;

    vec![
        ("User".to_string(), username),
        ("OS".to_string(), format!("{} {}", os_name, os_version)),
        ("Kernel".to_string(), kernel_version),
        (
            "Uptime".to_string(),
            format!("{} hours, {} mins", uptime_hours, uptime_minutes),
        ),
    ]
}
