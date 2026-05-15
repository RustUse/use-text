use crate::code_fence::{FenceDelimiter, is_closing_fence, parse_opening_fence};
use crate::frontmatter::frontmatter_line_count;
use crate::plain_text::inline_markdown_to_text;

/// A Markdown inline link.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarkdownLink {
    /// The cleaned label text.
    pub text: String,
    /// The destination target.
    pub target: String,
    /// The optional inline title.
    pub title: Option<String>,
    /// The 1-based line where the link was found.
    pub line: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InlineReferenceKind {
    Link,
    Image,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParsedInlineReference {
    pub label: String,
    pub target: String,
    pub title: Option<String>,
    pub line: usize,
}

/// Extracts inline links while ignoring fenced code blocks.
pub fn extract_links(markdown: &str) -> Vec<MarkdownLink> {
    extract_inline_references(markdown, InlineReferenceKind::Link)
        .into_iter()
        .map(|reference| MarkdownLink {
            text: reference.label,
            target: reference.target,
            title: reference.title,
            line: reference.line,
        })
        .collect()
}

pub(crate) fn extract_inline_references(
    markdown: &str,
    kind: InlineReferenceKind,
) -> Vec<ParsedInlineReference> {
    let frontmatter_lines = frontmatter_line_count(markdown);
    let mut references = Vec::new();
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

        let mut cursor = 0usize;
        let bytes = line.as_bytes();
        while cursor < bytes.len() {
            let matches_kind = match kind {
                InlineReferenceKind::Link => {
                    bytes[cursor] == b'[' && (cursor == 0 || bytes[cursor - 1] != b'!')
                },
                InlineReferenceKind::Image => {
                    bytes[cursor] == b'!' && bytes.get(cursor + 1) == Some(&b'[')
                },
            };

            if matches_kind
                && let Some((reference, next_cursor)) =
                    parse_inline_reference_at(line, cursor, kind, index + 1)
            {
                references.push(reference);
                cursor = next_cursor;
                continue;
            }

            cursor += 1;
        }
    }

    references
}

pub(crate) fn parse_inline_reference_at(
    line: &str,
    start: usize,
    kind: InlineReferenceKind,
    line_number: usize,
) -> Option<(ParsedInlineReference, usize)> {
    let (open_bracket, label_start) = match kind {
        InlineReferenceKind::Link => (start, start + 1),
        InlineReferenceKind::Image => {
            if line.as_bytes().get(start) != Some(&b'!') {
                return None;
            }
            (start + 1, start + 2)
        },
    };

    let label_end = find_matching_bracket(line, open_bracket)?;
    let mut cursor = label_end + 1;

    while line
        .as_bytes()
        .get(cursor)
        .is_some_and(u8::is_ascii_whitespace)
    {
        cursor += 1;
    }

    if line.as_bytes().get(cursor) != Some(&b'(') {
        return None;
    }

    let target_end = find_matching_paren(line, cursor)?;
    let label = inline_markdown_to_text(&line[label_start..label_end]);
    let (target, title) = parse_target_and_title(&line[cursor + 1..target_end])?;

    Some((
        ParsedInlineReference {
            label,
            target,
            title,
            line: line_number,
        },
        target_end + 1,
    ))
}

fn parse_target_and_title(input: &str) -> Option<(String, Option<String>)> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let (target_part, title_part) = if let Some(stripped) = trimmed.strip_prefix('<') {
        let close = stripped.find('>')?;
        (&stripped[..close], stripped[close + 1..].trim())
    } else {
        let split_index = find_target_split_index(trimmed);
        match split_index {
            Some(index) => (&trimmed[..index], trimmed[index..].trim()),
            None => (trimmed, ""),
        }
    };

    let target = target_part.trim();
    if target.is_empty() {
        return None;
    }

    let title = if title_part.is_empty() {
        None
    } else {
        parse_title_literal(title_part)
    };

    Some((target.to_owned(), title))
}

fn find_target_split_index(input: &str) -> Option<usize> {
    let mut depth = 0usize;

    for (index, character) in input.char_indices() {
        match character {
            '(' => depth += 1,
            ')' if depth > 0 => depth -= 1,
            character if character.is_whitespace() && depth == 0 => return Some(index),
            _ => {},
        }
    }

    None
}

fn parse_title_literal(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.len() < 2 {
        return None;
    }

    let first = trimmed.chars().next()?;
    let last = trimmed.chars().last()?;
    match (first, last) {
        ('"', '"') | ('\'', '\'') | ('(', ')') => {
            Some(trimmed[first.len_utf8()..trimmed.len() - last.len_utf8()].to_owned())
        },
        _ => None,
    }
}

fn find_matching_bracket(line: &str, open_index: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut depth = 0usize;
    let mut index = open_index;

    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 2,
            b'[' => {
                depth += 1;
                index += 1;
            },
            b']' => {
                depth = depth.saturating_sub(1);
                index += 1;
                if depth == 0 {
                    return Some(index - 1);
                }
            },
            _ => index += 1,
        }
    }

    None
}

fn find_matching_paren(line: &str, open_index: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut depth = 0usize;
    let mut quote = None;
    let mut index = open_index;

    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'\\' {
            index += 2;
            continue;
        }

        if let Some(active_quote) = quote {
            if byte == active_quote {
                quote = None;
            }
            index += 1;
            continue;
        }

        match byte {
            b'"' | b'\'' => {
                quote = Some(byte);
                index += 1;
            },
            b'(' => {
                depth += 1;
                index += 1;
            },
            b')' => {
                depth = depth.saturating_sub(1);
                index += 1;
                if depth == 0 {
                    return Some(index - 1);
                }
            },
            _ => index += 1,
        }
    }

    None
}
