use crate::code_fence::{FenceDelimiter, is_closing_fence, parse_opening_fence};
use crate::frontmatter::frontmatter_line_count;
use crate::heading::parse_heading_line;
use crate::link::{InlineReferenceKind, parse_inline_reference_at};

/// Converts Markdown into lightweight plain text.
pub fn markdown_to_plain_text(markdown: &str) -> String {
    let frontmatter_lines = frontmatter_line_count(markdown);
    let mut lines = Vec::new();
    let mut active_fence: Option<FenceDelimiter> = None;

    for (index, line) in markdown.lines().enumerate() {
        if index < frontmatter_lines {
            continue;
        }

        if let Some(delimiter) = active_fence {
            if is_closing_fence(line, delimiter) {
                active_fence = None;
                continue;
            }

            let trimmed = line.trim();
            if !trimmed.is_empty() {
                lines.push(trimmed.to_owned());
            }
            continue;
        }

        if let Some(opening) = parse_opening_fence(line) {
            active_fence = Some(opening.delimiter);
            continue;
        }

        if crate::is_horizontal_rule(line) {
            continue;
        }

        let mut candidate = strip_blockquote_markers(line);
        if let Some((_, heading_text)) = parse_heading_line(candidate) {
            candidate = heading_text;
        } else if let Some(content) = crate::ordered_list_item_content(candidate) {
            candidate = content;
        } else if let Some(content) = crate::unordered_list_item_content(candidate) {
            candidate = content;
        }

        let cleaned = inline_markdown_to_text(candidate);
        if !cleaned.is_empty() {
            lines.push(cleaned);
        }
    }

    lines.join("\n")
}

pub(crate) fn inline_markdown_to_text(input: &str) -> String {
    let mut output = String::new();
    let mut index = 0usize;
    let bytes = input.as_bytes();

    while index < bytes.len() {
        if bytes[index] == b'!'
            && bytes.get(index + 1) == Some(&b'[')
            && let Some((reference, next_index)) =
                parse_inline_reference_at(input, index, InlineReferenceKind::Image, 0)
        {
            output.push_str(reference.label.trim());
            index = next_index;
            continue;
        }

        if bytes[index] == b'['
            && (index == 0 || bytes[index - 1] != b'!')
            && let Some((reference, next_index)) =
                parse_inline_reference_at(input, index, InlineReferenceKind::Link, 0)
        {
            output.push_str(reference.label.trim());
            index = next_index;
            continue;
        }

        let Some(character) = input[index..].chars().next() else {
            break;
        };

        if character == '\\' {
            let next_index = index + character.len_utf8();
            if let Some(next_character) = input[next_index..].chars().next() {
                output.push(next_character);
                index = next_index + next_character.len_utf8();
            } else {
                index = next_index;
            }
            continue;
        }

        if matches!(character, '*' | '_' | '`' | '~') {
            index += character.len_utf8();
            continue;
        }

        output.push(character);
        index += character.len_utf8();
    }

    collapse_whitespace(&output)
}

fn strip_blockquote_markers(line: &str) -> &str {
    let mut candidate = line.trim_start();
    while let Some(stripped) = candidate.strip_prefix('>') {
        candidate = stripped.trim_start();
    }
    candidate
}

fn collapse_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}
