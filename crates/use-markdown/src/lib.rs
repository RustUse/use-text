#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod code_fence;
pub mod frontmatter;
pub mod heading;
pub mod image;
pub mod link;
pub mod outline;
pub mod plain_text;

pub use code_fence::{MarkdownCodeFence, extract_code_fences};
pub use frontmatter::{extract_frontmatter, has_frontmatter, strip_frontmatter};
pub use heading::{MarkdownHeading, extract_headings};
pub use image::{MarkdownImage, extract_images};
pub use link::{MarkdownLink, extract_links};
pub use outline::{MarkdownOutline, extract_outline};
pub use plain_text::markdown_to_plain_text;

/// Converts heading text into a practical GitHub-style anchor.
pub fn heading_to_anchor(text: &str) -> String {
    let mut anchor = String::new();
    let mut previous_was_separator = false;

    for character in text.trim().chars() {
        if character.is_alphanumeric() {
            for lowered in character.to_lowercase() {
                anchor.push(lowered);
            }
            previous_was_separator = false;
            continue;
        }

        if matches!(character, '\'' | '"' | '`') {
            continue;
        }

        if !anchor.is_empty() && !previous_was_separator {
            anchor.push('-');
            previous_was_separator = true;
        }
    }

    anchor.trim_matches('-').to_owned()
}

/// Returns `true` when a line looks like a Markdown horizontal rule.
pub fn is_horizontal_rule(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.len() < 3 {
        return false;
    }

    let mut marker = None;
    let mut count = 0usize;

    for character in trimmed.chars() {
        if character.is_whitespace() {
            continue;
        }

        if !matches!(character, '-' | '*' | '_') {
            return false;
        }

        match marker {
            Some(existing) if existing != character => return false,
            Some(_) => {},
            None => marker = Some(character),
        }

        count += 1;
    }

    count >= 3
}

/// Returns `true` when a line starts with a Markdown blockquote marker.
pub fn is_blockquote(line: &str) -> bool {
    line.trim_start().starts_with('>')
}

/// Returns `true` when a line starts with an unordered list marker.
pub fn is_unordered_list_item(line: &str) -> bool {
    unordered_list_item_content(line).is_some()
}

/// Returns `true` when a line starts with an ordered list marker.
pub fn is_ordered_list_item(line: &str) -> bool {
    ordered_list_item_content(line).is_some()
}

pub(crate) fn strip_up_to_three_leading_spaces(line: &str) -> &str {
    let mut removed = 0usize;
    let mut index = 0usize;

    for (byte_index, character) in line.char_indices() {
        if removed < 3 && character == ' ' {
            removed += 1;
            index = byte_index + character.len_utf8();
        } else {
            return &line[index..];
        }
    }

    &line[index..]
}

pub(crate) fn unordered_list_item_content(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    let mut characters = trimmed.chars();
    let marker = characters.next()?;
    if !matches!(marker, '-' | '*' | '+') {
        return None;
    }

    let marker_length = marker.len_utf8();
    let remainder = &trimmed[marker_length..];
    remainder
        .chars()
        .next()
        .filter(|character| character.is_whitespace())
        .map(|character| &remainder[character.len_utf8()..])
}

pub(crate) fn ordered_list_item_content(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    let digit_count = trimmed
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .count();
    if digit_count == 0 {
        return None;
    }

    let marker_start = trimmed
        .char_indices()
        .nth(digit_count)
        .map_or(trimmed.len(), |(index, _)| index);
    let marker = trimmed[marker_start..].chars().next()?;
    if !matches!(marker, '.' | ')') {
        return None;
    }

    let after_marker = &trimmed[marker_start + marker.len_utf8()..];
    after_marker
        .chars()
        .next()
        .filter(|character| character.is_whitespace())
        .map(|character| &after_marker[character.len_utf8()..])
}
