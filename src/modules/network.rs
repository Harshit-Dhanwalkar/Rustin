use std::process::Command;
use sysinfo::Networks;

pub fn get_network_info() -> Vec<(String, String)> {
    let networks = Networks::new_with_refreshed_list();
    let mut info = Vec::new();

    let network_ssid = get_network_ssid();
    let mut network_info = "No network".to_string();
    for (interface, data) in &networks {
        if data.total_received() > 0 || data.total_transmitted() > 0 {
            network_info = format!("{} ({})", network_ssid, interface.clone());
            break;
        }
    }

    info.push(("Network".to_string(), network_info));

    info
}

fn get_network_ssid() -> String {
    Command::new("sh")
        .arg("-c")
        .arg("nmcli -t -f active,ssid dev wifi | awk -F: '$1==\"yes\"{print $2}'")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}
