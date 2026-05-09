#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::collections::HashSet;

/// A normalized word token.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Word {
    text: String,
}

impl Word {
    /// Creates a normalized word when at least one word character is present.
    pub fn new(input: &str) -> Option<Self> {
        let text = normalize_word(input);
        (!text.is_empty()).then_some(Self { text })
    }

    /// Returns the normalized word text.
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Consumes the word and returns the owned string.
    pub fn into_string(self) -> String {
        self.text
    }
}

/// Aggregate counts derived from text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WordStats {
    /// Total number of normalized words.
    pub total: usize,
    /// Total number of distinct normalized words.
    pub unique: usize,
}

impl WordStats {
    /// Builds stats from the input text.
    pub fn from_text(input: &str) -> Self {
        let all_words = words(input);
        let total = all_words.len();
        let unique = unique_words(input).len();
        Self { total, unique }
    }
}

/// Counts normalized words in the input.
pub fn word_count(input: &str) -> usize {
    words(input).len()
}

/// Returns distinct normalized words in first-seen order.
pub fn unique_words(input: &str) -> Vec<Word> {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for word in words(input) {
        if seen.insert(word.text.clone()) {
            unique.push(word);
        }
    }

    unique
}

/// Normalizes a word-like value by lowercasing letters and dropping non-word punctuation.
pub fn normalize_word(input: &str) -> String {
    let characters: Vec<char> = input.trim().chars().collect();
    let mut output = String::new();

    for (index, character) in characters.iter().copied().enumerate() {
        let previous = index
            .checked_sub(1)
            .and_then(|value| characters.get(value))
            .copied();
        let next = characters.get(index + 1).copied();

        if character.is_alphanumeric() || is_apostrophe(previous, character, next) {
            output.extend(character.to_lowercase());
        }
    }

    output
}

/// Returns `true` when the input contains the target as a full normalized word.
pub fn contains_word(input: &str, target: &str) -> bool {
    let normalized_target = normalize_word(target);
    !normalized_target.is_empty()
        && words(input)
            .iter()
            .any(|word| word.as_str() == normalized_target)
}

/// Returns `true` when the first normalized word matches the target.
pub fn starts_with_word(input: &str, target: &str) -> bool {
    let normalized_target = normalize_word(target);
    !normalized_target.is_empty()
        && words(input)
            .first()
            .is_some_and(|word| word.as_str() == normalized_target)
}

/// Returns `true` when the last normalized word matches the target.
pub fn ends_with_word(input: &str, target: &str) -> bool {
    let normalized_target = normalize_word(target);
    !normalized_target.is_empty()
        && words(input)
            .last()
            .is_some_and(|word| word.as_str() == normalized_target)
}

/// Extracts normalized words from the input.
pub fn words(input: &str) -> Vec<Word> {
    word_ranges(input)
        .into_iter()
        .filter_map(|(start, end)| Word::new(&input[start..end]))
        .collect()
}

fn word_ranges(input: &str) -> Vec<(usize, usize)> {
    let characters: Vec<(usize, char)> = input.char_indices().collect();
    let mut ranges = Vec::new();
    let mut start = None;

    for (index, (byte_index, character)) in characters.iter().copied().enumerate() {
        let previous = index.checked_sub(1).map(|value| characters[value].1);
        let next = characters.get(index + 1).map(|(_, value)| *value);
        let is_word_character =
            character.is_alphanumeric() || is_apostrophe(previous, character, next);

        if is_word_character {
            if start.is_none() {
                start = Some(byte_index);
            }
        } else if let Some(word_start) = start.take() {
            ranges.push((word_start, byte_index));
        }
    }

    if let Some(word_start) = start {
        ranges.push((word_start, input.len()));
    }

    ranges
}

fn is_apostrophe(previous: Option<char>, current: char, next: Option<char>) -> bool {
    matches!(current, '\'' | '’')
        && previous.is_some_and(char::is_alphanumeric)
        && next.is_some_and(char::is_alphanumeric)
}

#[cfg(test)]
mod tests {
    use super::{
        Word, WordStats, contains_word, ends_with_word, normalize_word, starts_with_word,
        unique_words, word_count, words,
    };

    #[test]
    fn handles_empty_and_whitespace_only_input() {
        assert_eq!(word_count(""), 0);
        assert!(words("   \n").is_empty());
        assert_eq!(normalize_word("   \n"), "");
    }

    #[test]
    fn normalizes_ascii_words_and_punctuation() {
        assert_eq!(normalize_word("Hello!"), "hello");
        assert_eq!(normalize_word("don't"), "don't");
        assert_eq!(word_count("Hello, hello world"), 3);
    }

    #[test]
    fn preserves_first_seen_unique_words() {
        let unique = unique_words("Hello, hello world world");
        let texts: Vec<&str> = unique.iter().map(Word::as_str).collect();
        assert_eq!(texts, vec!["hello", "world"]);
    }

    #[test]
    fn checks_word_boundaries() {
        assert!(contains_word("Hello, world", "world"));
        assert!(!contains_word("cartwheel", "art"));
        assert!(starts_with_word("Hello world", "hello"));
        assert!(ends_with_word("Hello world!", "world"));
    }

    #[test]
    fn handles_multiline_and_unicode_input() {
        let extracted = words("Straße\ncafé");
        let texts: Vec<&str> = extracted.iter().map(Word::as_str).collect();
        assert_eq!(texts, vec!["straße", "café"]);

        let stats = WordStats::from_text("Straße\ncafé café");
        assert_eq!(stats.total, 3);
        assert_eq!(stats.unique, 2);
    }
}
