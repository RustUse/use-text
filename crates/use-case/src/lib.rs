#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::error::Error;
use std::fmt;

/// Describes a detected or requested text case.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextCase {
    /// Empty or whitespace-only input.
    Empty,
    /// Lowercase text without separators.
    Lower,
    /// Uppercase text without separators.
    Upper,
    /// Lowercase text separated by underscores.
    Snake,
    /// Lowercase text separated by hyphens.
    Kebab,
    /// Upper camel case such as `RustUse`.
    Pascal,
    /// Lower camel case such as `rustUse`.
    Camel,
    /// Title case separated by whitespace.
    Title,
    /// Uppercase text separated by underscores.
    Constant,
    /// Text that does not match a supported case shape.
    Mixed,
}

/// A reusable case-conversion descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaseConversion {
    source: TextCase,
    target: TextCase,
}

impl CaseConversion {
    /// Creates a new conversion descriptor.
    pub const fn new(source: TextCase, target: TextCase) -> Self {
        Self { source, target }
    }

    /// Returns the declared source case.
    pub const fn source(self) -> TextCase {
        self.source
    }

    /// Returns the target case.
    pub const fn target(self) -> TextCase {
        self.target
    }

    /// Applies the conversion to the provided input.
    pub fn apply(self, input: &str) -> Result<String, CaseError> {
        convert_to_case(input, self.target)
    }
}

/// Errors returned by [`CaseConversion`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaseError {
    /// The requested target case is descriptive rather than convertible.
    UnsupportedTarget(TextCase),
}

impl fmt::Display for CaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedTarget(case) => {
                write!(formatter, "unsupported conversion target: {case:?}")
            },
        }
    }
}

impl Error for CaseError {}

/// Converts input into `snake_case`.
pub fn to_snake_case(input: &str) -> String {
    join_words(&normalized_words(input), "_")
}

/// Converts input into `kebab-case`.
pub fn to_kebab_case(input: &str) -> String {
    join_words(&normalized_words(input), "-")
}

/// Converts input into `PascalCase`.
pub fn to_pascal_case(input: &str) -> String {
    split_words(input)
        .into_iter()
        .map(|word| titlecase_word(&word))
        .collect()
}

/// Converts input into `camelCase`.
pub fn to_camel_case(input: &str) -> String {
    let words = normalized_words(input);
    let Some((first, rest)) = words.split_first() else {
        return String::new();
    };

    let mut output = first.clone();
    for word in rest {
        output.push_str(&titlecase_word(word));
    }
    output
}

/// Converts input into title case separated by spaces.
pub fn to_title_case(input: &str) -> String {
    split_words(input)
        .into_iter()
        .map(|word| titlecase_word(&word))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Converts input into `CONSTANT_CASE`.
pub fn to_constant_case(input: &str) -> String {
    split_words(input)
        .into_iter()
        .map(|word| uppercase_word(&word))
        .collect::<Vec<_>>()
        .join("_")
}

/// Detects the most practical case shape for the input.
pub fn detect_case(input: &str) -> TextCase {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return TextCase::Empty;
    }

    if is_constant_case(trimmed) {
        return TextCase::Constant;
    }

    if is_snake_case(trimmed) {
        return TextCase::Snake;
    }

    if is_kebab_case(trimmed) {
        return TextCase::Kebab;
    }

    if is_title_case(trimmed) {
        return TextCase::Title;
    }

    if is_pascal_case(trimmed) {
        return TextCase::Pascal;
    }

    if is_camel_case(trimmed) {
        return TextCase::Camel;
    }

    if is_upper_case(trimmed) {
        return TextCase::Upper;
    }

    if is_lower_case(trimmed) {
        return TextCase::Lower;
    }

    TextCase::Mixed
}

fn convert_to_case(input: &str, target: TextCase) -> Result<String, CaseError> {
    let output = match target {
        TextCase::Empty => return Ok(String::new()),
        TextCase::Lower => lowercase_word(input.trim()),
        TextCase::Upper => uppercase_word(input.trim()),
        TextCase::Snake => to_snake_case(input),
        TextCase::Kebab => to_kebab_case(input),
        TextCase::Pascal => to_pascal_case(input),
        TextCase::Camel => to_camel_case(input),
        TextCase::Title => to_title_case(input),
        TextCase::Constant => to_constant_case(input),
        TextCase::Mixed => return Err(CaseError::UnsupportedTarget(TextCase::Mixed)),
    };

    Ok(output)
}

fn split_words(input: &str) -> Vec<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    let mut words = Vec::new();
    let mut chunk = String::new();

    for character in trimmed.chars() {
        if character.is_alphanumeric() {
            chunk.push(character);
        } else if !chunk.is_empty() {
            extend_chunk_words(&chunk, &mut words);
            chunk.clear();
        }
    }

    if !chunk.is_empty() {
        extend_chunk_words(&chunk, &mut words);
    }

    words
}

fn extend_chunk_words(chunk: &str, words: &mut Vec<String>) {
    let characters: Vec<char> = chunk.chars().collect();
    if characters.is_empty() {
        return;
    }

    let mut start = 0;
    for index in 1..characters.len() {
        let previous = characters[index - 1];
        let current = characters[index];
        let next = characters.get(index + 1).copied();

        if is_chunk_boundary(previous, current, next) {
            words.push(characters[start..index].iter().collect());
            start = index;
        }
    }

    words.push(characters[start..].iter().collect());
}

fn is_chunk_boundary(previous: char, current: char, next: Option<char>) -> bool {
    (previous.is_lowercase() && current.is_uppercase())
        || (previous.is_alphabetic() && current.is_numeric())
        || (previous.is_numeric() && current.is_alphabetic())
        || (previous.is_uppercase()
            && current.is_uppercase()
            && next.is_some_and(|candidate| candidate.is_lowercase()))
}

fn normalized_words(input: &str) -> Vec<String> {
    split_words(input)
        .into_iter()
        .map(|word| lowercase_word(&word))
        .collect()
}

fn join_words(words: &[String], separator: &str) -> String {
    words.join(separator)
}

fn lowercase_word(word: &str) -> String {
    word.chars().flat_map(char::to_lowercase).collect()
}

fn uppercase_word(word: &str) -> String {
    word.chars().flat_map(char::to_uppercase).collect()
}

fn titlecase_word(word: &str) -> String {
    let mut characters = word.chars();
    let Some(first) = characters.next() else {
        return String::new();
    };

    first
        .to_uppercase()
        .chain(characters.flat_map(char::to_lowercase))
        .collect()
}

fn is_snake_case(input: &str) -> bool {
    input.contains('_')
        && !input.starts_with('_')
        && !input.ends_with('_')
        && !input.contains("__")
        && input.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        })
}

fn is_constant_case(input: &str) -> bool {
    input.contains('_')
        && !input.starts_with('_')
        && !input.ends_with('_')
        && !input.contains("__")
        && input.chars().all(|character| {
            character.is_ascii_uppercase() || character.is_ascii_digit() || character == '_'
        })
}

fn is_kebab_case(input: &str) -> bool {
    input.contains('-')
        && !input.starts_with('-')
        && !input.ends_with('-')
        && !input.contains("--")
        && input.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}

fn is_title_case(input: &str) -> bool {
    input.contains(char::is_whitespace)
        && input.split_whitespace().all(|word| {
            let mut characters = word.chars();
            let Some(first) = characters.next() else {
                return false;
            };

            first.is_uppercase()
                && characters
                    .all(|character| !character.is_alphabetic() || character.is_lowercase())
        })
}

fn is_pascal_case(input: &str) -> bool {
    !input.contains(|character: char| !character.is_alphanumeric())
        && input.chars().next().is_some_and(char::is_uppercase)
        && !is_upper_case(input)
}

fn is_camel_case(input: &str) -> bool {
    !input.contains(|character: char| !character.is_alphanumeric())
        && input.chars().next().is_some_and(char::is_lowercase)
        && input.chars().any(char::is_uppercase)
}

fn is_lower_case(input: &str) -> bool {
    !input.contains(|character: char| !character.is_alphanumeric())
        && input
            .chars()
            .all(|character| !character.is_alphabetic() || character.is_lowercase())
}

fn is_upper_case(input: &str) -> bool {
    !input.contains(|character: char| !character.is_alphanumeric())
        && input
            .chars()
            .all(|character| !character.is_alphabetic() || character.is_uppercase())
}

#[cfg(test)]
mod tests {
    use super::{
        CaseConversion, CaseError, TextCase, detect_case, to_camel_case, to_constant_case,
        to_kebab_case, to_pascal_case, to_snake_case, to_title_case,
    };

    #[test]
    fn handles_empty_and_whitespace_only_input() {
        assert_eq!(to_snake_case(""), "");
        assert_eq!(to_kebab_case("   \t"), "");
        assert_eq!(detect_case("   \n"), TextCase::Empty);
    }

    #[test]
    fn converts_ascii_and_mixed_case_inputs() {
        assert_eq!(to_snake_case("HTTPServerError"), "http_server_error");
        assert_eq!(to_kebab_case("userProfile"), "user-profile");
        assert_eq!(to_pascal_case("user_profile"), "UserProfile");
        assert_eq!(to_camel_case("user-profile"), "userProfile");
        assert_eq!(to_title_case("user_profile id"), "User Profile Id");
        assert_eq!(to_constant_case("userProfile42"), "USER_PROFILE_42");
    }

    #[test]
    fn collapses_punctuation_and_repeated_separators() {
        assert_eq!(to_snake_case("hello---world"), "hello_world");
        assert_eq!(to_kebab_case("hello___world"), "hello-world");
        assert_eq!(
            to_title_case("  release...candidate  "),
            "Release Candidate"
        );
    }

    #[test]
    fn supports_unicode_case_conversions_conservatively() {
        assert_eq!(to_title_case("élan vital"), "Élan Vital");
        assert_eq!(to_snake_case("Straße Mode"), "straße_mode");
    }

    #[test]
    fn detects_supported_cases() {
        assert_eq!(detect_case("snake_case"), TextCase::Snake);
        assert_eq!(detect_case("kebab-case"), TextCase::Kebab);
        assert_eq!(detect_case("Title Case"), TextCase::Title);
        assert_eq!(detect_case("CamelCase"), TextCase::Pascal);
        assert_eq!(detect_case("camelCase"), TextCase::Camel);
        assert_eq!(detect_case("MIXED_case-value"), TextCase::Mixed);
    }

    #[test]
    fn case_conversion_reports_unsupported_targets() {
        let conversion = CaseConversion::new(TextCase::Snake, TextCase::Mixed);
        assert_eq!(
            conversion.apply("hello_world"),
            Err(CaseError::UnsupportedTarget(TextCase::Mixed))
        );
    }

    #[test]
    fn case_conversion_applies_supported_targets() {
        let conversion = CaseConversion::new(TextCase::Snake, TextCase::Pascal);
        assert_eq!(
            conversion.apply("hello_world"),
            Ok(String::from("HelloWorld"))
        );
        assert_eq!(conversion.source(), TextCase::Snake);
        assert_eq!(conversion.target(), TextCase::Pascal);
    }
}
