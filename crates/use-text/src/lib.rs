#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_case;
pub use use_line;
pub use use_slug;
pub use use_token;
pub use use_word;

pub mod prelude {
    pub use use_case::*;
    pub use use_line::*;
    pub use use_slug::*;
    pub use use_token::*;
    pub use use_word::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::crate_ready;

    #[test]
    fn facade_exposes_focused_crates() {
        assert!(crate_ready());
    }
}