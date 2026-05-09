//! Basic readability and text metric helpers.
//!
//! Sentence counting uses a simple punctuation heuristic based on `.`, `!`, and
//! `?`. Consecutive sentence-ending punctuation marks are counted as a single
//! sentence boundary.
//!
//! Reading time defaults to 200 words per minute and rounds up to at least one
//! minute for non-empty input.

const DEFAULT_WORDS_PER_MINUTE: usize = 200;

/// Counts whitespace-delimited words.
///
/// # Examples
///
/// ```rust
/// use use_readable::word_count;
///
/// assert_eq!(word_count("composable text primitives"), 3);
/// ```
#[must_use]
pub fn word_count(input: &str) -> usize {
    input.split_whitespace().count()
}

/// Counts Rust `char` values.
///
/// # Examples
///
/// ```rust
/// use use_readable::char_count;
///
/// assert_eq!(char_count("hé"), 2);
/// ```
#[must_use]
pub fn char_count(input: &str) -> usize {
    input.chars().count()
}

/// Counts UTF-8 bytes.
///
/// # Examples
///
/// ```rust
/// use use_readable::byte_count;
///
/// assert_eq!(byte_count("hé"), 3);
/// ```
#[must_use]
pub fn byte_count(input: &str) -> usize {
    input.len()
}

/// Counts sentences using `.`, `!`, and `?` as sentence-ending punctuation.
///
/// Consecutive terminal punctuation marks such as `...` or `?!` count as a
/// single sentence boundary.
///
/// # Examples
///
/// ```rust
/// use use_readable::sentence_count;
///
/// assert_eq!(sentence_count("One. Two! Three?"), 3);
/// assert_eq!(sentence_count("Wait... really?!"), 2);
/// ```
#[must_use]
pub fn sentence_count(input: &str) -> usize {
    let mut count = 0;
    let mut in_terminal_run = false;

    for ch in input.chars() {
        if matches!(ch, '.' | '!' | '?') {
            if !in_terminal_run {
                count += 1;
                in_terminal_run = true;
            }
        } else {
            in_terminal_run = false;
        }
    }

    count
}

/// Estimates reading time using a default speed of 200 words per minute.
///
/// Returns `0` for empty or whitespace-only input.
///
/// # Examples
///
/// ```rust
/// use use_readable::estimated_reading_time_minutes;
///
/// assert_eq!(estimated_reading_time_minutes("hello world"), 1);
/// ```
#[must_use]
pub fn estimated_reading_time_minutes(input: &str) -> usize {
    estimated_reading_time_minutes_at(input, DEFAULT_WORDS_PER_MINUTE).unwrap_or(0)
}

/// Estimates reading time using a custom words-per-minute value.
///
/// Returns `None` when `words_per_minute` is `0`.
///
/// # Examples
///
/// ```rust
/// use use_readable::estimated_reading_time_minutes_at;
///
/// assert_eq!(estimated_reading_time_minutes_at("one two three four", 2), Some(2));
/// assert_eq!(estimated_reading_time_minutes_at("", 200), Some(0));
/// assert_eq!(estimated_reading_time_minutes_at("hello", 0), None);
/// ```
#[must_use]
pub fn estimated_reading_time_minutes_at(input: &str, words_per_minute: usize) -> Option<usize> {
    if words_per_minute == 0 {
        return None;
    }

    if input.trim().is_empty() {
        return Some(0);
    }

    let words = word_count(input).max(1);
    Some(words.div_ceil(words_per_minute).max(1))
}

#[cfg(test)]
mod tests {
    use super::{
        byte_count, char_count, estimated_reading_time_minutes, estimated_reading_time_minutes_at,
        sentence_count, word_count,
    };

    #[test]
    fn counts_words_characters_and_bytes() {
        assert_eq!(word_count("composable text primitives"), 3);
        assert_eq!(char_count("hé"), 2);
        assert_eq!(byte_count("hé"), 3);
    }

    #[test]
    fn counts_sentences_with_simple_punctuation() {
        assert_eq!(sentence_count("One. Two! Three?"), 3);
        assert_eq!(sentence_count("Wait... really?!"), 2);
        assert_eq!(sentence_count("No punctuation here"), 0);
    }

    #[test]
    fn estimates_reading_time_with_default_speed() {
        assert_eq!(estimated_reading_time_minutes("one two three"), 1);
        assert_eq!(estimated_reading_time_minutes("   "), 0);
    }

    #[test]
    fn estimates_reading_time_with_custom_speed() {
        assert_eq!(
            estimated_reading_time_minutes_at("one two three four", 2),
            Some(2)
        );
        assert_eq!(estimated_reading_time_minutes_at("hello", 200), Some(1));
        assert_eq!(estimated_reading_time_minutes_at("", 200), Some(0));
    }

    #[test]
    fn rejects_invalid_words_per_minute() {
        assert_eq!(estimated_reading_time_minutes_at("hello", 0), None);
    }
}
