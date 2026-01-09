#[must_use]
pub fn truncate(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else if max_length > 3 {
        format!("{}...", &text[..max_length - 3])
    } else {
        String::new()
    }
}

#[must_use]
pub fn wrap(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return lines;
    }

    let mut current_line = String::new();

    for word in words {
        let word_length = word.len();
        let space_length = usize::from(!current_line.is_empty());

        if current_line.len() + space_length + word_length <= max_width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
            }

            if word_length <= max_width {
                current_line.push_str(word);
            } else {
                current_line.push_str(&word[..max_width.saturating_sub(3)]);
                current_line.push_str("...");
            }
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
