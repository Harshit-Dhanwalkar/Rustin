use colored::*;
use std::env;
use std::fs;
use std::process::Command;

mod modules;
mod utils;

use modules::*;
use utils::*;

fn main() {
    print_system_info();
}

fn print_system_info() {
    let logo = match fs::read_to_string("logo.txt") {
        Ok(content) => content.lines().map(|s| s.to_string()).collect(),
        Err(_) => {
            eprintln!("Warning: Could not read logo.txt. Using default logo.");
            vec![
                r"    __    __ ".to_string(),
                r"   / /_  / /_".to_string(),
                r"  / __ \/ __/".to_string(),
                r" / / / / /_  ".to_string(),
                r"/_/ /_/\__/  ".to_string(),
            ]
        }
    };

    let mut info = Vec::new();

    // System information
    info.extend(system::get_system_info());

    // Software information
    info.extend(software::get_software_info());

    // Network information
    info.extend(network::get_network_info());

    // Terminal information
    info.extend(terminal::get_terminal_info());

    // Display information
    info.extend(display::get_display_info());

    // Hardware information
    info.extend(hardware::get_hardware_info());

    // Calculate maximum lengths for dynamic padding
    let max_label_len = info.iter().map(|(label, _)| label.len()).max().unwrap_or(0);
    let logo_width = logo.iter().map(|line| line.len()).max().unwrap_or(0);

    // Calculate the width of the main content block
    let mut max_info_line_len = 0;
    for (label, value) in &info {
        let line = format!(
            "{:>label_width$} {}",
            label,
            value,
            label_width = max_label_len
        );
        let visible_len = visible_width(&line);
        if visible_len > max_info_line_len {
            max_info_line_len = visible_len;
        }
    }

    // Calculate the total width needed
    let horizontal_padding = 1;
    let total_content_width = max_info_line_len + horizontal_padding + logo_width;
    let border_width = total_content_width + 1;

    let top_left = "┌".blue().bold();
    let top_right = "┐".blue().bold();
    let bottom_left = "└".blue().bold();
    let bottom_right = "┘".blue().bold();
    let horizontal = "─".blue().bold();
    let vertical = "│".blue().bold();

    // Top border
    println!(
        "{}{}{}",
        top_left,
        horizontal.to_string().repeat(border_width),
        top_right
    );

    // Print content lines
    for i in 0..logo.len().max(info.len()) {
        let info_line = if i < info.len() {
            let (label, value) = &info[i];
            format!(
                "{} {:<label_width$} {}",
                vertical,
                label.blue().bold(),
                value,
                label_width = max_label_len
            )
        } else {
            format!("{} {}", vertical, " ".repeat(max_info_line_len))
        };

        let logo_line = if i < logo.len() {
            logo[i].to_string()
        } else {
            " ".repeat(logo_width)
        };

        let info_visible_width = visible_width(&info_line);
        let padding_needed =
            max_info_line_len.saturating_sub(info_visible_width) + horizontal_padding;

        println!(
            "{} {} {} {}",
            info_line,
            " ".repeat(padding_needed),
            logo_line,
            vertical
        );
    }

    // Bottom border
    println!(
        "{}{}{}",
        bottom_left,
        horizontal.to_string().repeat(border_width),
        bottom_right
    );
}
