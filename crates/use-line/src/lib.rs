#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A numbered logical line.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Line {
    /// The 1-based line number.
    pub number: LineNumber,
    /// The line contents without the line ending.
    pub text: String,
}

/// A 1-based line number.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineNumber(usize);

impl LineNumber {
    /// Creates a new 1-based line number.
    #[must_use]
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    /// Returns the raw 1-based number.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

/// Aggregate line counts derived from text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineStats {
    /// Total number of logical lines.
    pub total: usize,
    /// Number of logical lines whose trimmed content is not empty.
    pub non_empty: usize,
}

impl LineStats {
    /// Builds stats from the input text.
    #[must_use]
    pub fn from_text(input: &str) -> Self {
        Self {
            total: line_count(input),
            non_empty: non_empty_line_count(input),
        }
    }
}

/// Supported line-ending shapes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineEnding {
    /// `\n`
    Lf,
    /// `\r\n`
    Crlf,
    /// `\r`
    Cr,
}

impl LineEnding {
    /// Returns the concrete line-ending string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lf => "\n",
            Self::Crlf => "\r\n",
            Self::Cr => "\r",
        }
    }
}

/// Counts logical lines, ignoring a trailing empty line created only by a final line ending.
#[must_use]
pub fn line_count(input: &str) -> usize {
    logical_lines(input).len()
}

/// Counts logical lines whose trimmed content is not empty.
#[must_use]
pub fn non_empty_line_count(input: &str) -> usize {
    logical_lines(input)
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .count()
}

/// Trims each logical line independently and preserves the input line-ending style when possible.
#[must_use]
pub fn trim_lines(input: &str) -> String {
    transform_lines(input, |line| line.trim().to_owned())
}

/// Normalizes line endings to the requested target.
#[must_use]
pub fn normalize_line_endings(input: &str, ending: LineEnding) -> String {
    normalize_to_lf(input).replace('\n', ending.as_str())
}

/// Prefixes each logical line with the provided indent string.
#[must_use]
pub fn indent_lines(input: &str, indent: &str) -> String {
    transform_lines(input, |line| {
        let mut output = String::with_capacity(indent.len() + line.len());
        output.push_str(indent);
        output.push_str(line);
        output
    })
}

/// Removes the common indentation shared by non-empty lines.
#[must_use]
pub fn dedent_lines(input: &str) -> String {
    let ending = preferred_line_ending(input);
    let normalized = normalize_to_lf(input);
    let mut lines: Vec<String> = normalized.split('\n').map(ToOwned::to_owned).collect();

    if normalized.is_empty() {
        lines.clear();
    }

    let indent = common_indent(&lines);
    if indent.is_empty() {
        return lines.join(ending.as_str());
    }

    for line in &mut lines {
        if line.trim().is_empty() {
            line.clear();
        } else if line.starts_with(&indent) {
            line.drain(..indent.len());
        }
    }

    lines.join(ending.as_str())
}

/// Returns numbered logical lines.
#[must_use]
pub fn lines_with_numbers(input: &str) -> Vec<Line> {
    logical_lines(input)
        .into_iter()
        .enumerate()
        .map(|(index, text)| Line {
            number: LineNumber::new(index + 1),
            text,
        })
        .collect()
}

fn transform_lines<F>(input: &str, mut transform: F) -> String
where
    F: FnMut(&str) -> String,
{
    let ending = preferred_line_ending(input);
    let normalized = normalize_to_lf(input);
    if normalized.is_empty() {
        return String::new();
    }

    normalized
        .split('\n')
        .map(&mut transform)
        .collect::<Vec<_>>()
        .join(ending.as_str())
}

fn normalize_to_lf(input: &str) -> String {
    input.replace("\r\n", "\n").replace('\r', "\n")
}

fn preferred_line_ending(input: &str) -> LineEnding {
    if input.contains("\r\n") {
        LineEnding::Crlf
    } else if input.contains('\r') {
        LineEnding::Cr
    } else {
        LineEnding::Lf
    }
}

fn logical_lines(input: &str) -> Vec<String> {
    let normalized = normalize_to_lf(input);
    if normalized.is_empty() {
        return Vec::new();
    }

    let mut lines: Vec<String> = normalized.split('\n').map(ToOwned::to_owned).collect();
    if normalized.ends_with('\n') {
        let _ = lines.pop();
    }
    lines
}

fn common_indent(lines: &[String]) -> String {
    let mut non_empty = lines.iter().filter(|line| !line.trim().is_empty());
    let Some(first) = non_empty.next() else {
        return String::new();
    };

    let mut common: Vec<char> = leading_indent(first).chars().collect();
    for line in non_empty {
        let indent: Vec<char> = leading_indent(line).chars().collect();
        let shared = common
            .iter()
            .zip(indent.iter())
            .take_while(|(left, right)| left == right)
            .count();
        common.truncate(shared);
        if common.is_empty() {
            break;
        }
    }

    common.into_iter().collect()
}

fn leading_indent(line: &str) -> &str {
    let end = line
        .char_indices()
        .find(|(_, character)| *character != ' ' && *character != '\t')
        .map_or(line.len(), |(index, _)| index);
    &line[..end]
}

#[cfg(test)]
mod tests {
    use super::{
        LineEnding, LineStats, dedent_lines, indent_lines, line_count, lines_with_numbers,
        non_empty_line_count, normalize_line_endings, trim_lines,
    };

    #[test]
    fn counts_empty_and_whitespace_only_inputs() {
        assert_eq!(line_count(""), 0);
        assert_eq!(line_count("\n"), 1);
        assert_eq!(non_empty_line_count("  \n\t"), 0);
    }

    #[test]
    fn trims_and_normalizes_multiline_text() {
        assert_eq!(trim_lines(" alpha \n beta "), "alpha\nbeta");
        assert_eq!(
            normalize_line_endings("a\r\nb\rc\n", LineEnding::Lf),
            "a\nb\nc\n"
        );
        assert_eq!(normalize_line_endings("a\nb", LineEnding::Crlf), "a\r\nb");
    }

    #[test]
    fn indents_and_dedents_lines() {
        assert_eq!(indent_lines("alpha\nbeta", "  "), "  alpha\n  beta");
        assert_eq!(
            dedent_lines("    alpha\n      beta\n    \n    gamma"),
            "alpha\n  beta\n\ngamma"
        );
    }

    #[test]
    fn numbers_lines_and_builds_stats() {
        let lines = lines_with_numbers("alpha\nbeta\n");
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].number.get(), 1);
        assert_eq!(lines[1].text, "beta");

        let stats = LineStats::from_text("alpha\n\n beta ");
        assert_eq!(stats.total, 3);
        assert_eq!(stats.non_empty, 2);
    }

    #[test]
    fn handles_unicode_text_without_special_cases() {
        assert_eq!(trim_lines(" café \n Straße "), "café\nStraße");
    }
}
