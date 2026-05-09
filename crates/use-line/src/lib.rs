#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Placeholder until the full use-line API is implemented.
pub fn crate_ready() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::crate_ready;

    #[test]
    fn crate_scaffold_is_ready() {
        assert!(crate_ready());
    }
}