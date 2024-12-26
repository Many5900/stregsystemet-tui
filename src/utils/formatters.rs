pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        let safe_length = max_length.saturating_sub(3);

        format!("{}...", &text[0..safe_length])
    }
}

pub fn format_error_message(message: &str, max_width: usize, max_lines: usize) -> String {
    let words: Vec<&str> = message.split_whitespace().collect();

    let mut formatted_lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if !current_line.is_empty() && current_line.len() + word.len() + 1 > max_width {
            formatted_lines.push(current_line);
            current_line = word.to_string();
        } else if current_line.is_empty() {
            current_line = word.to_string();
        } else {
            current_line.push(' ');
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        formatted_lines.push(current_line);
    }

    if formatted_lines.len() > max_lines {
        formatted_lines.truncate(max_lines - 1);
        formatted_lines.push("...".to_string());
    }

    formatted_lines.join("\n")
}

pub fn sanitize_html(input: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    let mut tag_content = String::new();

    for c in input.chars() {
        if c == '<' {
            in_tag = true;
            tag_content.clear();
        } else if c == '>' {
            in_tag = false;

            let tag_name = tag_content.split_whitespace().next().unwrap_or("");
            let block_tags = [
                "h1", "h2", "h3", "h4", "h5", "h6", "p", "div", "br", "/h1", "/h2", "/h3", "/h4",
                "/h5", "/h6", "/p", "/div",
            ];

            if block_tags.contains(&tag_name) && !result.is_empty() && !result.ends_with(' ') {
                result.push(' ');
            }
        } else if in_tag {
            tag_content.push(c);
        } else {
            result.push(c);
        }
    }

    let mut normalized = String::new();
    let mut last_was_space = false;

    for c in result.chars() {
        if c.is_whitespace() {
            if !last_was_space {
                normalized.push(' ');
                last_was_space = true;
            }
        } else {
            normalized.push(c);
            last_was_space = false;
        }
    }

    let mut final_result = String::new();
    let chars: Vec<char> = normalized.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c == '(' && i > 0 && chars[i - 1].is_alphanumeric() {
            final_result.push(' ');
        }
        final_result.push(c);
    }

    final_result.trim().to_string()
}
