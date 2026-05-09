//! Whitespace and simple text normalization helpers.
//!
//! `normalize_whitespace` trims the full string and collapses all whitespace runs
//! to a single ASCII space.
//!
//! `collapse_whitespace` performs the same collapse without trimming outer runs,
//! so leading or trailing whitespace becomes a single leading or trailing space.

/// Collapses all whitespace runs to single spaces and trims the result.
///
/// # Examples
///
/// ```rust
/// use use_normalize::normalize_whitespace;
///
/// assert_eq!(normalize_whitespace("  composable   text\n\nprimitives  "), "composable text primitives");
/// ```
#[must_use]
pub fn normalize_whitespace(input: &str) -> String {
    collapse_whitespace(input).trim().to_string()
}

/// Collapses all whitespace runs to single spaces without trimming the result.
///
/// # Examples
///
/// ```rust
/// use use_normalize::collapse_whitespace;
///
/// assert_eq!(collapse_whitespace("  hello\nworld  "), " hello world ");
/// ```
#[must_use]
pub fn collapse_whitespace(input: &str) -> String {
    let mut result = String::new();
    let mut last_was_whitespace = false;

    for ch in input.chars() {
        if ch.is_whitespace() {
            if !last_was_whitespace {
                result.push(' ');
                last_was_whitespace = true;
            }
        } else {
            result.push(ch);
            last_was_whitespace = false;
        }
    }

    result
}

/// Trims leading and trailing whitespace from every line.
///
/// # Examples
///
/// ```rust
/// use use_normalize::trim_lines;
///
/// assert_eq!(trim_lines("  one  \n\t two\t\n"), "one\ntwo\n");
/// ```
#[must_use]
pub fn trim_lines(input: &str) -> String {
    input.split('\n').map(str::trim).collect::<Vec<_>>().join("\n")
}

/// Removes lines that are empty after trimming and returns trimmed lines.
///
/// # Examples
///
/// ```rust
/// use use_normalize::remove_empty_lines;
///
/// assert_eq!(remove_empty_lines(" one \n\n  \n two "), "one\ntwo");
/// ```
#[must_use]
pub fn remove_empty_lines(input: &str) -> String {
    input
        .split('\n')
        .filter_map(|line| {
            let trimmed = line.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Ensures the string ends with exactly one trailing newline.
///
/// # Examples
///
/// ```rust
/// use use_normalize::ensure_single_trailing_newline;
///
/// assert_eq!(ensure_single_trailing_newline("hello"), "hello\n");
/// assert_eq!(ensure_single_trailing_newline("hello\n\n"), "hello\n");
/// ```
#[must_use]
pub fn ensure_single_trailing_newline(input: &str) -> String {
    let mut result = input.trim_end_matches('\n').to_string();
    result.push('\n');
    result
}

#[cfg(test)]
mod tests {
    use super::{
        collapse_whitespace, ensure_single_trailing_newline, normalize_whitespace,
        remove_empty_lines, trim_lines,
    };

    #[test]
    fn normalizes_whitespace() {
        assert_eq!(normalize_whitespace("  composable   text\n\nprimitives  "), "composable text primitives");
        assert_eq!(normalize_whitespace("\t spaced \t"), "spaced");
    }

    #[test]
    fn collapses_whitespace_without_trimming() {
        assert_eq!(collapse_whitespace("  hello\nworld  "), " hello world ");
        assert_eq!(collapse_whitespace("a\t\tb"), "a b");
    }

    #[test]
    fn trims_each_line() {
        assert_eq!(trim_lines("  one  \n\t two\t\n"), "one\ntwo\n");
        assert_eq!(trim_lines(" line "), "line");
    }

    #[test]
    fn removes_empty_lines_after_trimming() {
        assert_eq!(remove_empty_lines(" one \n\n  \n two "), "one\ntwo");
        assert_eq!(remove_empty_lines("\n\t\n"), "");
    }

    #[test]
    fn ensures_exactly_one_trailing_newline() {
        assert_eq!(ensure_single_trailing_newline("hello"), "hello\n");
        assert_eq!(ensure_single_trailing_newline("hello\n\n"), "hello\n");
        assert_eq!(ensure_single_trailing_newline(""), "\n");
    }
}
