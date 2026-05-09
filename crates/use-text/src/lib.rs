#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_case;
pub use use_line;
pub use use_slug;
pub use use_token;
pub use use_word;

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::{LineEnding, slugify, to_snake_case, token_count, word_count};

    #[test]
    fn facade_exposes_focused_crates() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(token_count("Hello world"), 2);
        assert_eq!(word_count("Hello world"), 2);
        assert_eq!(LineEnding::Lf.as_str(), "\n");
    }
}
