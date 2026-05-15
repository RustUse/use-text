use crate::frontmatter::frontmatter_line_count;

/// A fenced code block extracted from Markdown.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarkdownCodeFence {
    /// The optional info-string language token.
    pub language: Option<String>,
    /// The raw fenced content without the opening or closing fence lines.
    pub content: String,
    /// The 1-based line where the opening fence appears.
    pub start_line: usize,
    /// The 1-based line where the closing fence appears, or the last line when unclosed.
    pub end_line: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FenceDelimiter {
    pub character: char,
    pub length: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FenceOpening<'a> {
    pub delimiter: FenceDelimiter,
    pub info: &'a str,
}

/// Extracts fenced code blocks delimited by triple backticks or triple tildes.
pub fn extract_code_fences(markdown: &str) -> Vec<MarkdownCodeFence> {
    let mut fences = Vec::new();
    let frontmatter_lines = frontmatter_line_count(markdown);
    let total_lines = markdown.lines().count();
    let mut current: Option<(FenceDelimiter, Option<String>, usize, Vec<String>)> = None;

    for (index, line) in markdown.lines().enumerate() {
        if index < frontmatter_lines {
            continue;
        }

        let line_number = index + 1;

        if let Some((delimiter, _, _, _)) = current {
            if is_closing_fence(line, delimiter) {
                let (delimiter, language, start_line, content) = current
                    .take()
                    .expect("fence state should exist when closing");
                let _ = delimiter;
                fences.push(MarkdownCodeFence {
                    language,
                    content: content.join("\n"),
                    start_line,
                    end_line: line_number,
                });
                continue;
            }

            if let Some((_, _, _, content)) = current.as_mut() {
                content.push(line.to_owned());
            }
            continue;
        }

        if let Some(opening) = parse_opening_fence(line) {
            current = Some((
                opening.delimiter,
                language_from_info(opening.info),
                line_number,
                Vec::new(),
            ));
        }
    }

    if let Some((_, language, start_line, content)) = current {
        fences.push(MarkdownCodeFence {
            language,
            content: content.join("\n"),
            start_line,
            end_line: total_lines.max(start_line),
        });
    }

    fences
}

pub(crate) fn parse_opening_fence(line: &str) -> Option<FenceOpening<'_>> {
    let candidate = crate::strip_up_to_three_leading_spaces(line);
    let marker = candidate.chars().next()?;
    if !matches!(marker, '`' | '~') {
        return None;
    }

    let count = candidate
        .chars()
        .take_while(|character| *character == marker)
        .count();
    if count < 3 {
        return None;
    }

    let info_start = count * marker.len_utf8();
    Some(FenceOpening {
        delimiter: FenceDelimiter {
            character: marker,
            length: count,
        },
        info: candidate[info_start..].trim(),
    })
}

pub(crate) fn is_closing_fence(line: &str, delimiter: FenceDelimiter) -> bool {
    let candidate = crate::strip_up_to_three_leading_spaces(line);
    let count = candidate
        .chars()
        .take_while(|character| *character == delimiter.character)
        .count();

    if count < delimiter.length {
        return false;
    }

    candidate[count * delimiter.character.len_utf8()..]
        .trim()
        .is_empty()
}

fn language_from_info(info: &str) -> Option<String> {
    info.split_whitespace().next().map(ToOwned::to_owned)
}
