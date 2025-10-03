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
                "  ___".to_string(),
                " (o o)".to_string(),
                "(  V  )".to_string(),
                "--m-m-".to_string(),
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

    // Get terminal size
    let (term_width, term_height) = get_terminal_size();

    // Format ASCII art for display based on terminal height
    let max_art_height = term_height.saturating_sub(4); // Leave space for borders
    let formatted_art = format_ascii_art_for_display(&logo, max_art_height);

    // Calculate layout based on terminal width
    let (display_art, _) = calculate_layout(&formatted_art, &info, term_width);
    let max_label_len = info
        .iter()
        .map(|(label, _)| visible_width(label))
        .max()
        .unwrap_or(0);

    // Calculate art width
    let art_width = display_art
        .iter()
        .map(|line| visible_width(line))
        .max()
        .unwrap_or(0);

    // Calculate info width
    let mut max_info_width = 0;
    for (label, value) in &info {
        let line_width = max_label_len + 2 + visible_width(value); // label + ": " + value
        if line_width > max_info_width {
            max_info_width = line_width;
        }
    }
    let padding = 1; // Space between art and info
    let total_width_needed = art_width + padding + max_info_width;
    let use_side_by_side = total_width_needed <= term_width.saturating_sub(4); // Account for borders
    if use_side_by_side {
        // Side-by-side layout
        display_side_by_side(&display_art, &info, max_label_len, art_width, term_width);
    } else {
        // Stacked layout
        display_stacked(&display_art, &info, max_label_len, term_width);
    }
}

fn display_side_by_side(
    ascii_art: &[String],
    info: &[(String, String)],
    max_label_len: usize,
    art_width: usize,
    term_width: usize,
) {
    let top_left = "┌".blue().bold();
    let top_right = "┐".blue().bold();
    let bottom_left = "└".blue().bold();
    let bottom_right = "┘".blue().bold();
    let horizontal = "─".blue().bold();
    let vertical = "│".blue().bold();

    // Calculate content width (without borders)
    let content_width = term_width.saturating_sub(2);
    let padding = 3;

    // Top border
    println!(
        "{}{}{}",
        top_left,
        horizontal.to_string().repeat(content_width),
        top_right
    );

    let max_lines = ascii_art.len().max(info.len());

    for i in 0..max_lines {
        let mut line_content = String::new();
        if i < info.len() {
            let (label, value) = &info[i];
            let info_line = format!(
                "{} {:<width$} {}",
                vertical,
                label.blue().bold(),
                value,
                width = max_label_len
            );
            line_content.push_str(&info_line);
        } else {
            let empty_info = format!("{} {}", vertical, " ".repeat(max_label_len + 2));
            line_content.push_str(&empty_info);
        }

        // Calculate padding between info and art
        let current_info_width = visible_width(&line_content);
        let padding_needed = content_width.saturating_sub(current_info_width + art_width);

        if padding_needed > padding {
            line_content.push_str(&" ".repeat(padding_needed));
        } else {
            line_content.push_str(&" ".repeat(padding));
        }

        if i < ascii_art.len() {
            line_content.push_str(&ascii_art[i]);
        } else {
            line_content.push_str(&" ".repeat(art_width));
        }

        let current_width = visible_width(&line_content);
        let remaining_space = content_width.saturating_sub(current_width);
        line_content.push_str(&" ".repeat(remaining_space));
        line_content.push_str(&vertical.to_string());

        println!("{}", line_content);
    }

    // Bottom border
    println!(
        "{}{}{}",
        bottom_left,
        horizontal.to_string().repeat(content_width),
        bottom_right
    );
}

fn display_stacked(
    ascii_art: &[String],
    info: &[(String, String)],
    max_label_len: usize,
    term_width: usize,
) {
    let top_left = "┌".blue().bold();
    let top_right = "┐".blue().bold();
    let bottom_left = "└".blue().bold();
    let bottom_right = "┘".blue().bold();
    let horizontal = "─".blue().bold();
    let vertical = "│".blue().bold();

    let content_width = term_width.saturating_sub(2);

    // Top border
    println!(
        "{}{}{}",
        top_left,
        horizontal.to_string().repeat(content_width),
        top_right
    );

    // Display ASCII art (centered)
    for line in ascii_art {
        let line_width = visible_width(line);
        let padding = (content_width.saturating_sub(line_width)) / 2;
        let right_padding = content_width.saturating_sub(padding + line_width);

        println!(
            "{} {}{}{} {}",
            vertical,
            " ".repeat(padding),
            line,
            " ".repeat(right_padding),
            vertical
        );
    }

    if !ascii_art.is_empty() {
        println!("{} {} {}", vertical, " ".repeat(content_width), vertical);
    }

    // Display info lines
    for (label, value) in info {
        let info_line = format!(
            "{} {:<width$} {}",
            vertical,
            label.blue().bold(),
            value,
            width = max_label_len
        );

        let info_width = visible_width(&info_line);
        let padding = content_width.saturating_sub(info_width - 2);

        println!("{}{} {}", info_line, " ".repeat(padding), vertical);
    }

    // Bottom border
    println!(
        "{}{}{}",
        bottom_left,
        horizontal.to_string().repeat(content_width),
        bottom_right
    );
}
