use std::fs;
use unicode_width::UnicodeWidthStr;

pub fn break_long_text(text: &str, max_line_length: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_line_length && !current_line.is_empty() {
            lines.push(current_line.trim().to_string());
            current_line = String::new();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        lines.push(current_line.trim().to_string());
    }

    lines
}

pub fn visible_width(s: &str) -> usize {
    let stripped = strip_ansi_escapes::strip_str(s);
    UnicodeWidthStr::width(stripped.as_str())
}

pub fn get_version(cmd: &str, args: &[&str]) -> Option<String> {
    use regex::Regex;
    use std::process::Command;

    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                let re = Regex::new(r"(\d+\.\d+(\.\d+)?)").unwrap();
                re.captures(&version_str)
                    .map(|caps| caps[1].to_string())
                    .or_else(|| {
                        version_str
                            .lines()
                            .next()
                            .and_then(|line| line.split_whitespace().nth(1).map(|s| s.to_string()))
                    })
            } else {
                None
            }
        })
}

pub fn read_ascii_art(file_path: &str) -> Option<Vec<String>> {
    fs::read_to_string(file_path)
        .ok()
        .map(|content| content.lines().map(|line| line.to_string()).collect())
}

pub fn get_terminal_size() -> (usize, usize) {
    if let Some((width, height)) = term_size::dimensions() {
        (width, height)
    } else {
        (80, 24)
    }
}

pub fn format_ascii_art_for_display(ascii_art: &[String], max_height: usize) -> Vec<String> {
    let art_height = ascii_art.len();

    if art_height <= max_height {
        ascii_art.to_vec()
    } else {
        ascii_art.iter().take(max_height).cloned().collect()
    }
}

pub fn calculate_layout(
    ascii_art: &[String],
    info_lines: &[(String, String)],
    terminal_width: usize,
) -> (Vec<String>, Vec<String>) {
    let art_width = ascii_art
        .iter()
        .map(|line| visible_width(line))
        .max()
        .unwrap_or(0);

    let info_width = info_lines
        .iter()
        .map(|(label, value)| visible_width(label) + visible_width(value) + 2) // +2 for ": " spacing
        .max()
        .unwrap_or(0);

    let padding = 4; // Space between art and info

    let total_width_needed = art_width + padding + info_width;

    if total_width_needed <= terminal_width {
        (ascii_art.to_vec(), vec![])
    } else {
        let available_art_width = terminal_width.saturating_sub(info_width + padding);

        let cropped_art: Vec<String> = if available_art_width > 0 {
            ascii_art
                .iter()
                .map(|line| {
                    if visible_width(line) > available_art_width {
                        // Truncate line to fit available width
                        let mut result = String::new();
                        let mut current_width = 0;

                        for ch in line.chars() {
                            let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
                            if current_width + ch_width <= available_art_width {
                                result.push(ch);
                                current_width += ch_width;
                            } else {
                                break;
                            }
                        }
                        result
                    } else {
                        line.clone()
                    }
                })
                .collect()
        } else {
            vec![]
        };

        (cropped_art, vec![])
    }
}
