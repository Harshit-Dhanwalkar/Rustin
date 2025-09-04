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
