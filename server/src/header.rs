use crate::comments::CommentDelimiters;
use chrono::Local;
use regex::Regex;

pub const ASCII_ART: [&str; 7] = [
    "        :::      ::::::::",
    "      :+:      :+:    :+:",
    "    +:+ +:+         +:+  ",
    "  +#+  +:+       +#+     ",
    "+#+#+#+#+#+   +#+        ",
    "     #+#    #+#          ",
    "    ###   ########.fr    ",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderInfo {
    pub filename: String,
    pub author_user: String,
    pub author_mail: String,
    pub created_date: String,
    pub created_user: String,
    pub updated_date: String,
    pub updated_user: String,
}

fn text_line(left: &str, right: &str, delimiters: &CommentDelimiters) -> String {
    let available = if delimiters.max_width >= 10 {
        delimiters.max_width - 10
    } else {
        0
    };

    let left_max_len = if available >= right.len() {
        available - right.len()
    } else {
        0
    };

    let truncated_left = if left.len() > left_max_len {
        &left[..left_max_len]
    } else {
        left
    };

    let spaces_count = if available >= truncated_left.len() + right.len() {
        available - truncated_left.len() - right.len()
    } else {
        0
    };

    let left_margin = 5usize.saturating_sub(delimiters.start.len());
    let right_margin = 5usize.saturating_sub(delimiters.end.len());

    format!(
        "{}{}{}{}{}{}{}",
        delimiters.start,
        " ".repeat(left_margin),
        truncated_left,
        " ".repeat(spaces_count),
        right,
        " ".repeat(right_margin),
        delimiters.end
    )
}

pub fn generate_line(
    n: usize,
    filename: &str,
    user: &str,
    mail: &str,
    date: &str,
    delimiters: &CommentDelimiters,
) -> String {
    match n {
        1 | 11 => {
            let fill_len = delimiters
                .max_width
                .saturating_sub(delimiters.start.len())
                .saturating_sub(delimiters.end.len())
                .saturating_sub(2);
            format!(
                "{} {} {}",
                delimiters.start,
                delimiters.fill.to_string().repeat(fill_len),
                delimiters.end
            )
        }
        2 | 10 => text_line("", "", delimiters),
        3 | 5 | 7 => text_line("", ASCII_ART[n - 3], delimiters),
        4 => text_line(filename, ASCII_ART[1], delimiters),
        6 => {
            let full_author = format!("By: {} <{}>", user, mail);
            let available = delimiters
                .max_width
                .saturating_sub(10)
                .saturating_sub(ASCII_ART[3].len());
            let author = if full_author.len() > available {
                format!("By: {}", mail)
            } else {
                full_author
            };
            text_line(&author, ASCII_ART[3], delimiters)
        }
        8 => {
            let created = format!("Created: {} by {}", date, user);
            text_line(&created, ASCII_ART[5], delimiters)
        }
        9 => {
            let updated = format!("Updated: {} by {}", date, user);
            text_line(&updated, ASCII_ART[6], delimiters)
        }
        _ => String::new(),
    }
}

pub fn generate_header(
    filename: &str,
    user: &str,
    mail: &str,
    created: &str,
    updated: &str,
    delimiters: &CommentDelimiters,
) -> String {
    let mut lines = Vec::with_capacity(11);
    for n in 1..=11 {
        let date = if n == 8 { created } else { updated };
        lines.push(generate_line(n, filename, user, mail, date, delimiters));
    }
    lines.join("\n")
}

pub fn current_formatted_date() -> String {
    Local::now().format("%Y/%m/%d %H:%M:%S").to_string()
}

pub fn detect_header(content: &str, delimiters: &CommentDelimiters) -> Option<HeaderInfo> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 11 {
        return None;
    }

    // Quick structural checks
    let line0 = lines[0].trim();
    let line10 = lines[10].trim();
    if !line0.starts_with(delimiters.start) || !line0.ends_with(delimiters.end) {
        return None;
    }
    if !line10.starts_with(delimiters.start) || !line10.ends_with(delimiters.end) {
        return None;
    }

    let left_margin = 5usize.saturating_sub(delimiters.start.len());
    let prefix = format!("{}{}", delimiters.start, " ".repeat(left_margin));

    if !lines[7].starts_with(&prefix) || !lines[7].contains("Created: ") {
        return None;
    }
    if !lines[8].starts_with(&prefix) || !lines[8].contains("Updated: ") {
        return None;
    }

    // Extract filename from line 3 (index 3)
    let escaped_start = regex::escape(delimiters.start);
    let re_file_pat = format!(r"^\s*(?:{}\s*)([^\s]+)", escaped_start);
    let re_file = Regex::new(&re_file_pat).ok()?;
    let filename = if let Some(caps) = re_file.captures(lines[3]) {
        caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default()
    } else {
        String::new()
    };

    // Extract created line info
    let re_created = Regex::new(r"Created:\s+([0-9]{4}/[0-9]{2}/[0-9]{2}\s+[0-9]{2}:[0-9]{2}:[0-9]{2})\s+by\s+([^\s]+)").ok()?;
    let (created_date, created_user) = if let Some(caps) = re_created.captures(lines[7]) {
        (
            caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default(),
            caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default(),
        )
    } else {
        (String::new(), String::new())
    };

    // Extract updated line info
    let re_updated = Regex::new(r"Updated:\s+([0-9]{4}/[0-9]{2}/[0-9]{2}\s+[0-9]{2}:[0-9]{2}:[0-9]{2})\s+by\s+([^\s]+)").ok()?;
    let (updated_date, updated_user) = if let Some(caps) = re_updated.captures(lines[8]) {
        (
            caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default(),
            caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default(),
        )
    } else {
        (String::new(), String::new())
    };

    // Extract author info
    let re_author = Regex::new(r"By:\s+([^\s<]+)(?:\s+<([^>]+)>)?").ok()?;
    let (author_user, author_mail) = if let Some(caps) = re_author.captures(lines[5]) {
        (
            caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default(),
            caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default(),
        )
    } else {
        (String::new(), String::new())
    };

    Some(HeaderInfo {
        filename,
        author_user,
        author_mail,
        created_date,
        created_user,
        updated_date,
        updated_user,
    })
}

pub fn update_header_timestamp(
    content: &str,
    user: &str,
    delimiters: &CommentDelimiters,
) -> Option<String> {
    let header_info = detect_header(content, delimiters)?;
    let now = current_formatted_date();
    let new_line_9 = generate_line(
        9,
        &header_info.filename,
        user,
        &header_info.author_mail,
        &now,
        delimiters,
    );

    let lines: Vec<&str> = content.lines().collect();
    let has_trailing_newline = content.ends_with('\n');

    let owned_lines: Vec<String> = lines
        .iter()
        .enumerate()
        .map(|(i, &line)| {
            if i == 8 {
                new_line_9.clone()
            } else {
                line.to_string()
            }
        })
        .collect();

    let mut result = owned_lines.join("\n");
    if has_trailing_newline {
        result.push('\n');
    }
    Some(result)
}
