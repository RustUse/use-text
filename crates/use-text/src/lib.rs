#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_case;
pub use use_line;
pub use use_markdown;
pub use use_slug;
pub use use_token;
pub use use_word;

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::{
        LineEnding, extract_headings, heading_to_anchor, slugify, to_snake_case, token_count,
        word_count,
    };

    #[test]
    fn facade_exposes_focused_crates() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(extract_headings("# Hello")[0].text, "Hello");
        assert_eq!(heading_to_anchor("Hello World"), "hello-world");
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(token_count("Hello world"), 2);
        assert_eq!(word_count("Hello world"), 2);
        assert_eq!(LineEnding::Lf.as_str(), "\n");
    }
}
