/// Extracts top-of-document frontmatter contents without the boundary lines.
pub fn extract_frontmatter(markdown: &str) -> Option<&str> {
    let bounds = detect_frontmatter_bounds(markdown)?;
    Some(&markdown[bounds.content_start..bounds.content_end])
}

/// Returns `true` when the document starts with YAML-like or TOML-like frontmatter.
pub fn has_frontmatter(markdown: &str) -> bool {
    detect_frontmatter_bounds(markdown).is_some()
}

/// Returns the document without a leading frontmatter block.
pub fn strip_frontmatter(markdown: &str) -> &str {
    detect_frontmatter_bounds(markdown)
        .map(|bounds| &markdown[bounds.full_end..])
        .unwrap_or(markdown)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FrontmatterBounds {
    pub content_start: usize,
    pub content_end: usize,
    pub full_end: usize,
    pub line_count: usize,
}

pub(crate) fn frontmatter_line_count(markdown: &str) -> usize {
    detect_frontmatter_bounds(markdown)
        .map(|bounds| bounds.line_count)
        .unwrap_or(0)
}

pub(crate) fn detect_frontmatter_bounds(markdown: &str) -> Option<FrontmatterBounds> {
    let bom_length = if markdown.starts_with('\u{feff}') {
        '\u{feff}'.len_utf8()
    } else {
        0
    };

    let (_, first_line_end, first_line) = next_line(markdown, bom_length)?;
    let fence = match first_line.trim() {
        "---" => "---",
        "+++" => "+++",
        _ => return None,
    };

    let content_start = first_line_end;
    let mut line_count = 1usize;
    let mut offset = first_line_end;

    while let Some((line_start, line_end, line)) = next_line(markdown, offset) {
        line_count += 1;
        if line.trim() == fence {
            let content_end = trim_trailing_line_breaks(&markdown[content_start..line_start]).len()
                + content_start;
            return Some(FrontmatterBounds {
                content_start,
                content_end,
                full_end: line_end,
                line_count,
            });
        }

        offset = line_end;
    }

    None
}

fn trim_trailing_line_breaks(input: &str) -> &str {
    input.trim_end_matches(['\r', '\n'])
}

fn next_line(markdown: &str, start: usize) -> Option<(usize, usize, &str)> {
    if start >= markdown.len() {
        return None;
    }

    let remainder = &markdown[start..];
    if let Some(relative_end) = remainder.find('\n') {
        let line_end = start + relative_end + 1;
        let mut content_end = start + relative_end;
        if relative_end > 0 && remainder.as_bytes()[relative_end - 1] == b'\r' {
            content_end -= 1;
        }

        return Some((start, line_end, &markdown[start..content_end]));
    }

    Some((start, markdown.len(), &markdown[start..]))
}
