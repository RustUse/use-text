use crate::heading::{MarkdownHeading, extract_headings};

/// A simple heading outline for a Markdown document.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MarkdownOutline {
    /// The headings found in document order.
    pub headings: Vec<MarkdownHeading>,
}

/// Extracts a lightweight document outline from headings.
pub fn extract_outline(markdown: &str) -> MarkdownOutline {
    MarkdownOutline {
        headings: extract_headings(markdown),
    }
}
