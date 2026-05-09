//! ASCII-oriented case conversion helpers.
//!
//! These helpers normalize common ASCII separators and mixed-case inputs such as
//! `hello world`, `hello-world`, `hello_world`, `HelloWorld`, `helloWorld`, and
//! `HTTPServer`.
//!
//! Word boundary detection is intentionally ASCII-oriented for v0.1. Non-ASCII
//! letters are preserved as part of words, while separator detection and acronym
//! splitting focus on ASCII transitions.

fn words(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();
    let mut result = Vec::new();
    let mut current = String::new();

    for (index, &ch) in chars.iter().enumerate() {
        if !ch.is_alphanumeric() {
            if !current.is_empty() {
                result.push(std::mem::take(&mut current));
            }
            continue;
        }

        let should_split = if current.is_empty() {
            false
        } else {
            let previous = chars[index - 1];
            let next = chars.get(index + 1).copied();

            (ch.is_uppercase() && (previous.is_lowercase() || previous.is_numeric()))
                || (ch.is_uppercase()
                    && previous.is_uppercase()
                    && next.is_some_and(char::is_lowercase))
        };

        if should_split {
            result.push(std::mem::take(&mut current));
        }

        current.push(ch);
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

fn lowercase(word: &str) -> String {
    word.chars().flat_map(char::to_lowercase).collect()
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };

    let mut result = String::new();
    result.extend(first.to_uppercase());
    result.extend(chars.flat_map(char::to_lowercase));
    result
}

/// Converts text to `snake_case`.
///
/// # Examples
///
/// ```rust
/// use use_case::to_snake_case;
///
/// assert_eq!(to_snake_case("HTTP Server Config"), "http_server_config");
/// assert_eq!(to_snake_case("helloWorld"), "hello_world");
/// ```
#[must_use]
pub fn to_snake_case(input: &str) -> String {
    words(input)
        .into_iter()
        .map(|word| lowercase(&word))
        .collect::<Vec<_>>()
        .join("_")
}

/// Converts text to `kebab-case`.
///
/// # Examples
///
/// ```rust
/// use use_case::to_kebab_case;
///
/// assert_eq!(to_kebab_case("hello_world"), "hello-world");
/// assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
/// ```
#[must_use]
pub fn to_kebab_case(input: &str) -> String {
    words(input)
        .into_iter()
        .map(|word| lowercase(&word))
        .collect::<Vec<_>>()
        .join("-")
}

/// Converts text to `camelCase`.
///
/// # Examples
///
/// ```rust
/// use use_case::to_camel_case;
///
/// assert_eq!(to_camel_case("hello world"), "helloWorld");
/// assert_eq!(to_camel_case("HTTPServer"), "httpServer");
/// ```
#[must_use]
pub fn to_camel_case(input: &str) -> String {
    let mut words = words(input).into_iter();
    let Some(first) = words.next() else {
        return String::new();
    };

    let mut result = lowercase(&first);
    for word in words {
        result.push_str(&capitalize(&word));
    }
    result
}

/// Converts text to `PascalCase`.
///
/// # Examples
///
/// ```rust
/// use use_case::to_pascal_case;
///
/// assert_eq!(to_pascal_case("hello world"), "HelloWorld");
/// assert_eq!(to_pascal_case("http_server"), "HttpServer");
/// ```
#[must_use]
pub fn to_pascal_case(input: &str) -> String {
    words(input)
        .into_iter()
        .map(|word| capitalize(&word))
        .collect::<String>()
}

/// Converts text to `Title Case`.
///
/// # Examples
///
/// ```rust
/// use use_case::to_title_case;
///
/// assert_eq!(to_title_case("hello-world"), "Hello World");
/// assert_eq!(to_title_case("HTTPServer"), "Http Server");
/// ```
#[must_use]
pub fn to_title_case(input: &str) -> String {
    words(input)
        .into_iter()
        .map(|word| capitalize(&word))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::{to_camel_case, to_kebab_case, to_pascal_case, to_snake_case, to_title_case};

    #[test]
    fn converts_to_snake_case() {
        assert_eq!(to_snake_case("hello world"), "hello_world");
        assert_eq!(to_snake_case("hello-world"), "hello_world");
        assert_eq!(to_snake_case("hello_world"), "hello_world");
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("helloWorld"), "hello_world");
        assert_eq!(to_snake_case("HTTPServer"), "http_server");
        assert_eq!(to_snake_case("__Hello---World__"), "hello_world");
    }

    #[test]
    fn converts_to_kebab_case() {
        assert_eq!(to_kebab_case("hello world"), "hello-world");
        assert_eq!(to_kebab_case("hello_world"), "hello-world");
        assert_eq!(to_kebab_case("HTTPServer"), "http-server");
    }

    #[test]
    fn converts_to_camel_case() {
        assert_eq!(to_camel_case("hello world"), "helloWorld");
        assert_eq!(to_camel_case("hello-world"), "helloWorld");
        assert_eq!(to_camel_case("HTTPServer"), "httpServer");
    }

    #[test]
    fn converts_to_pascal_case() {
        assert_eq!(to_pascal_case("hello world"), "HelloWorld");
        assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
        assert_eq!(to_pascal_case("HTTPServer"), "HttpServer");
    }

    #[test]
    fn converts_to_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("hello-world"), "Hello World");
        assert_eq!(to_title_case("HTTPServer"), "Http Server");
    }

    #[test]
    fn returns_empty_output_for_empty_input() {
        assert_eq!(to_snake_case(""), "");
        assert_eq!(to_kebab_case("---"), "");
        assert_eq!(to_camel_case("___"), "");
        assert_eq!(to_pascal_case("   "), "");
        assert_eq!(to_title_case("\n\t"), "");
    }
}
