use crate::code_fence::{FenceDelimiter, is_closing_fence, parse_opening_fence};
use crate::frontmatter::frontmatter_line_count;
use crate::heading_to_anchor;
use crate::plain_text::inline_markdown_to_text;

/// A Markdown ATX heading.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarkdownHeading {
    /// The heading level from 1 to 6.
    pub level: u8,
    /// The cleaned heading text.
    pub text: String,
    /// The 1-based line where the heading was found.
    pub line: usize,
    /// A practical anchor derived from the heading text.
    pub anchor: String,
}

/// Extracts ATX headings while ignoring content inside fenced code blocks.
pub fn extract_headings(markdown: &str) -> Vec<MarkdownHeading> {
    let frontmatter_lines = frontmatter_line_count(markdown);
    let mut headings = Vec::new();
    let mut active_fence: Option<FenceDelimiter> = None;

    for (index, line) in markdown.lines().enumerate() {
        if index < frontmatter_lines {
            continue;
        }

        if let Some(delimiter) = active_fence {
            if is_closing_fence(line, delimiter) {
                active_fence = None;
            }
            continue;
        }

        if let Some(opening) = parse_opening_fence(line) {
            active_fence = Some(opening.delimiter);
            continue;
        }

        if let Some((level, text)) = parse_heading_line(line) {
            let text = inline_markdown_to_text(text);
            headings.push(MarkdownHeading {
                level,
                anchor: heading_to_anchor(&text),
                text,
                line: index + 1,
            });
        }
    }

    headings
}

pub(crate) fn parse_heading_line(line: &str) -> Option<(u8, &str)> {
    let candidate = crate::strip_up_to_three_leading_spaces(line);
    let level = candidate
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if !(1..=6).contains(&level) {
        return None;
    }

    let remainder = &candidate[level..];
    if !remainder.is_empty() && !remainder.chars().next().is_some_and(char::is_whitespace) {
        return None;
    }

    Some((level as u8, trim_closing_hashes(remainder.trim())))
}

fn trim_closing_hashes(text: &str) -> &str {
    let trimmed = text.trim();
    let mut end = trimmed.len();

    while end > 0 && trimmed.as_bytes()[end - 1] == b'#' {
        end -= 1;
    }

    if end == trimmed.len() {
        return trimmed;
    }

    let prefix = &trimmed[..end];
    if prefix.chars().last().is_some_and(char::is_whitespace) {
        prefix.trim_end()
    } else {
        trimmed
    }
}
