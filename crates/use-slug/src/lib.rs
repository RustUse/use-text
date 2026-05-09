//! Simple ASCII slug generation helpers.
//!
//! Slugs are lowercased, retain ASCII letters and digits, collapse runs of
//! non-alphanumeric separators, and trim leading or trailing separators.
//!
//! Unicode transliteration is intentionally out of scope for v0.1. Non-ASCII
//! characters are dropped rather than normalized into ASCII equivalents.

fn slugify_inner(input: &str, separator: char) -> String {
    let mut slug = String::new();
    let mut pending_separator = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_separator && !slug.is_empty() {
                slug.push(separator);
            }
            pending_separator = false;
            slug.push(ch.to_ascii_lowercase());
        } else if ch.is_ascii() {
            pending_separator = !slug.is_empty();
        }
    }

    slug
}

/// Generates a slug using `-` as the separator.
///
/// # Examples
///
/// ```rust
/// use use_slug::slugify;
///
/// assert_eq!(slugify("RustUse: Composable Text Primitives!"), "rustuse-composable-text-primitives");
/// assert_eq!(slugify("  hello---world  "), "hello-world");
/// ```
#[must_use]
pub fn slugify(input: &str) -> String {
    slugify_with_separator(input, '-')
}

/// Generates a slug using a custom separator.
///
/// # Examples
///
/// ```rust
/// use use_slug::slugify_with_separator;
///
/// assert_eq!(slugify_with_separator("Hello, World!", '_'), "hello_world");
/// assert_eq!(slugify_with_separator("naïve café", '-'), "na-ve-caf");
/// ```
#[must_use]
pub fn slugify_with_separator(input: &str, separator: char) -> String {
    slugify_inner(input, separator)
}

#[cfg(test)]
mod tests {
    use super::{slugify, slugify_with_separator};

    #[test]
    fn generates_default_slugs() {
        assert_eq!(slugify("RustUse: Composable Text Primitives!"), "rustuse-composable-text-primitives");
        assert_eq!(slugify("  hello---world  "), "hello-world");
        assert_eq!(slugify("hello_world"), "hello-world");
    }

    #[test]
    fn removes_punctuation_and_duplicate_separators() {
        assert_eq!(slugify("Hello,,, world!!!"), "hello-world");
        assert_eq!(slugify("---hello   world---"), "hello-world");
    }

    #[test]
    fn supports_custom_separators() {
        assert_eq!(slugify_with_separator("Hello, World!", '_'), "hello_world");
        assert_eq!(slugify_with_separator("a/b/c", '.'), "a.b.c");
    }

    #[test]
    fn drops_non_ascii_characters() {
        assert_eq!(slugify("naïve café"), "na-ve-caf");
        assert_eq!(slugify("你好，世界"), "");
    }

    #[test]
    fn trims_leading_and_trailing_separators() {
        assert_eq!(slugify("___hello___"), "hello");
        assert_eq!(slugify_with_separator("***hello***", '_'), "hello");
    }
}
