#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::fmt;

/// A validated default-separator slug.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Slug(String);

impl Slug {
    /// Creates a slug from an existing normalized value.
    pub fn new(value: &str) -> Option<Self> {
        is_slug(value).then(|| Self(value.to_owned()))
    }

    /// Normalizes free-form text into a slug with the default options.
    pub fn from_text(input: &str) -> Self {
        Self(slugify(input))
    }

    /// Normalizes free-form text into a slug with custom options.
    pub fn from_text_with_options(input: &str, options: SlugOptions) -> Self {
        Self(slugify_with_options(input, options))
    }

    /// Returns the slug text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the slug and returns the owned string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Slug {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Configures conservative slug shaping.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SlugOptions {
    /// Separator used between normalized words.
    pub separator: SlugSeparator,
    /// Optional maximum output length.
    pub max_length: Option<usize>,
}

impl Default for SlugOptions {
    fn default() -> Self {
        Self {
            separator: SlugSeparator::Dash,
            max_length: None,
        }
    }
}

/// Supported separators for generated slugs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SlugSeparator {
    /// `-`
    Dash,
    /// `_`
    Underscore,
    /// Any other non-alphanumeric ASCII separator.
    Custom(char),
}

impl SlugSeparator {
    /// Returns the separator character.
    pub const fn as_char(self) -> char {
        match self {
            Self::Dash => '-',
            Self::Underscore => '_',
            Self::Custom(character) => character,
        }
    }
}

/// Converts free-form text into a default slug.
pub fn slugify(input: &str) -> String {
    slugify_with_options(input, SlugOptions::default())
}

/// Normalizes a candidate slug using the default separator.
pub fn normalize_slug(input: &str) -> String {
    slugify(input)
}

/// Returns `true` when the input is already a normalized default slug.
pub fn is_slug(input: &str) -> bool {
    !input.is_empty()
        && input == input.trim()
        && !input.starts_with('-')
        && !input.ends_with('-')
        && !input.contains("--")
        && input
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-')
}

/// Returns the normalized slug segments.
pub fn slug_words(input: &str) -> Vec<String> {
    let slug = slugify(input);
    if slug.is_empty() {
        return Vec::new();
    }

    slug.split('-').map(ToOwned::to_owned).collect()
}

/// Truncates a slug without leaving trailing separators when possible.
pub fn truncate_slug(input: &str, max_length: usize) -> String {
    let slug = slugify(input);
    truncate_normalized_slug(&slug, max_length, '-')
}

fn slugify_with_options(input: &str, options: SlugOptions) -> String {
    let separator = normalize_separator(options.separator);
    let mut output = String::new();
    let mut previous_was_separator = false;

    for character in input.trim().chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
            previous_was_separator = false;
            continue;
        }

        if character.is_whitespace() || (character.is_ascii() && !character.is_ascii_alphanumeric()) {
            if !output.is_empty() && !previous_was_separator {
                output.push(separator);
                previous_was_separator = true;
            }
        }
    }

    while output.ends_with(separator) {
        output.pop();
    }

    if let Some(max_length) = options.max_length {
        return truncate_normalized_slug(&output, max_length, separator);
    }

    output
}

fn normalize_separator(separator: SlugSeparator) -> char {
    let candidate = separator.as_char();
    if candidate.is_ascii_alphanumeric() || candidate.is_ascii_whitespace() {
        '-'
    } else {
        candidate
    }
}

fn truncate_normalized_slug(slug: &str, max_length: usize, separator: char) -> String {
    if max_length == 0 || slug.is_empty() {
        return String::new();
    }

    if slug.len() <= max_length {
        return slug.to_owned();
    }

    let prefix = &slug[..max_length];
    if let Some(index) = prefix.rfind(separator) {
        let candidate = prefix[..index].trim_end_matches(separator);
        if !candidate.is_empty() {
            return candidate.to_owned();
        }
    }

    prefix.trim_end_matches(separator).to_owned()
}

#[cfg(test)]
mod tests {
    use super::{
        Slug, SlugOptions, SlugSeparator, is_slug, normalize_slug, slug_words, slugify,
        truncate_slug,
    };

    #[test]
    fn handles_empty_and_whitespace_only_input() {
        assert_eq!(slugify(""), "");
        assert_eq!(slugify("   \n"), "");
        assert_eq!(slug_words("   \n"), Vec::<String>::new());
    }

    #[test]
    fn normalizes_ascii_text_and_repeated_separators() {
        assert_eq!(slugify("Release Candidate 1"), "release-candidate-1");
        assert_eq!(slugify("release---candidate___1"), "release-candidate-1");
        assert_eq!(normalize_slug("  release_candidate  "), "release-candidate");
    }

    #[test]
    fn validates_and_truncates_slugs() {
        assert!(is_slug("release-candidate-1"));
        assert!(!is_slug("Release-Candidate-1"));
        assert_eq!(truncate_slug("release candidate patch", 14), "release");
        assert_eq!(truncate_slug("release candidate patch", 20), "release-candidate");
    }

    #[test]
    fn exposes_slug_words_and_custom_options() {
        assert_eq!(slug_words("Release Candidate"), vec![String::from("release"), String::from("candidate")]);

        let slug = Slug::from_text_with_options(
            "Release Candidate",
            SlugOptions {
                separator: SlugSeparator::Underscore,
                max_length: None,
            },
        );

        assert_eq!(slug.as_str(), "release_candidate");
    }

    #[test]
    fn treats_unicode_conservatively() {
        assert_eq!(slugify("Élan vital"), "lan-vital");
        assert_eq!(slugify("naïve façade"), "nave-faade");
    }

    #[test]
    fn slug_type_validates_normalized_values() {
        assert!(Slug::new("release-candidate").is_some());
        assert!(Slug::new("Release Candidate").is_none());
    }
}