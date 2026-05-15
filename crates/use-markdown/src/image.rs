use crate::link::{InlineReferenceKind, extract_inline_references};

/// A Markdown inline image.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarkdownImage {
    /// The cleaned alt text.
    pub alt: String,
    /// The image source.
    pub source: String,
    /// The optional inline title.
    pub title: Option<String>,
    /// The 1-based line where the image was found.
    pub line: usize,
}

/// Extracts inline images while ignoring fenced code blocks.
pub fn extract_images(markdown: &str) -> Vec<MarkdownImage> {
    extract_inline_references(markdown, InlineReferenceKind::Image)
        .into_iter()
        .map(|reference| MarkdownImage {
            alt: reference.label,
            source: reference.target,
            title: reference.title,
            line: reference.line,
        })
        .collect()
}
