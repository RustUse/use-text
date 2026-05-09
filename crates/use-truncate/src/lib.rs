//! Character- and word-safe text truncation helpers.
//!
//! `truncate_chars` and `truncate_with_ellipsis` operate on Rust `char` values,
//! never raw bytes, so UTF-8 boundaries remain intact.
//!
//! Word-based helpers split on whitespace. When truncation occurs they join the
//! retained words with single spaces for deterministic output.

const ELLIPSIS: char = '…';

fn take_chars(input: &str, count: usize) -> String {
    input.chars().take(count).collect()
}

/// Truncates text to at most `max_chars` Rust `char` values.
///
/// # Examples
///
/// ```rust
/// use use_truncate::truncate_chars;
///
/// assert_eq!(truncate_chars("abcdef", 4), "abcd");
/// assert_eq!(truncate_chars("héllo", 4), "héll");
/// ```
#[must_use]
pub fn truncate_chars(input: &str, max_chars: usize) -> String {
    take_chars(input, max_chars)
}

/// Truncates text to at most `max_words` whitespace-delimited words.
///
/// # Examples
///
/// ```rust
/// use use_truncate::truncate_words;
///
/// assert_eq!(truncate_words("one two three", 2), "one two");
/// assert_eq!(truncate_words(" one\n two\tthree ", 2), "one two");
/// ```
#[must_use]
pub fn truncate_words(input: &str, max_words: usize) -> String {
    if max_words == 0 {
        return String::new();
    }

    input.split_whitespace().take(max_words).collect::<Vec<_>>().join(" ")
}

/// Truncates text by `char` count and appends `…` when truncation occurs.
///
/// The returned string is never longer than `max_chars` characters. If
/// `max_chars` is `0`, an empty string is returned. If `max_chars` is `1`, the
/// result is just `…` when truncation is needed.
///
/// # Examples
///
/// ```rust
/// use use_truncate::truncate_with_ellipsis;
///
/// assert_eq!(truncate_with_ellipsis("abcdef", 4), "abc…");
/// assert_eq!(truncate_with_ellipsis("short", 10), "short");
/// ```
#[must_use]
pub fn truncate_with_ellipsis(input: &str, max_chars: usize) -> String {
    let char_count = input.chars().count();
    if char_count <= max_chars {
        return input.to_string();
    }

    match max_chars {
        0 => String::new(),
        1 => ELLIPSIS.to_string(),
        _ => {
            let mut truncated = take_chars(input, max_chars - 1);
            truncated.push(ELLIPSIS);
            truncated
        }
    }
}

/// Truncates text by word count and appends `…` when truncation occurs.
///
/// # Examples
///
/// ```rust
/// use use_truncate::truncate_words_with_ellipsis;
///
/// assert_eq!(truncate_words_with_ellipsis("one two three", 2), "one two…");
/// assert_eq!(truncate_words_with_ellipsis("one two", 2), "one two");
/// ```
#[must_use]
pub fn truncate_words_with_ellipsis(input: &str, max_words: usize) -> String {
    if max_words == 0 {
        return String::new();
    }

    let words: Vec<&str> = input.split_whitespace().collect();
    if words.len() <= max_words {
        return input.to_string();
    }

    let mut truncated = words.into_iter().take(max_words).collect::<Vec<_>>().join(" ");
    truncated.push(ELLIPSIS);
    truncated
}

#[cfg(test)]
mod tests {
    use super::{
        truncate_chars, truncate_with_ellipsis, truncate_words, truncate_words_with_ellipsis,
    };

    #[test]
    fn truncates_by_characters_without_splitting_bytes() {
        assert_eq!(truncate_chars("abcdef", 4), "abcd");
        assert_eq!(truncate_chars("héllo", 4), "héll");
        assert_eq!(truncate_chars("🙂🙂🙂", 2), "🙂🙂");
    }

    #[test]
    fn truncates_by_words() {
        assert_eq!(truncate_words("one two three", 2), "one two");
        assert_eq!(truncate_words(" one\n two\tthree ", 2), "one two");
        assert_eq!(truncate_words("one two", 0), "");
    }

    #[test]
    fn adds_ellipsis_for_character_truncation() {
        assert_eq!(truncate_with_ellipsis("abcdef", 4), "abc…");
        assert_eq!(truncate_with_ellipsis("abcdef", 1), "…");
        assert_eq!(truncate_with_ellipsis("abcdef", 0), "");
        assert_eq!(truncate_with_ellipsis("short", 10), "short");
    }

    #[test]
    fn adds_ellipsis_for_word_truncation() {
        assert_eq!(truncate_words_with_ellipsis("one two three", 2), "one two…");
        assert_eq!(truncate_words_with_ellipsis("one two", 2), "one two");
        assert_eq!(truncate_words_with_ellipsis("one two", 0), "");
    }

    #[test]
    fn handles_unicode_input_safely() {
        assert_eq!(truncate_with_ellipsis("Grüße aus Köln", 7), "Grüße …");
        assert_eq!(truncate_words_with_ellipsis("こんにちは 世界 Rust", 2), "こんにちは 世界…");
    }
}
