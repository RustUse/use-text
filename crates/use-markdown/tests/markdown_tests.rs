use use_markdown::{
    extract_code_fences, extract_frontmatter, extract_headings, extract_images, extract_links,
    extract_outline, has_frontmatter, heading_to_anchor, is_blockquote, is_horizontal_rule,
    is_ordered_list_item, is_unordered_list_item, markdown_to_plain_text, strip_frontmatter,
};

#[test]
fn extracts_headings_with_levels_lines_and_anchors() {
    let markdown = "# Intro\n\n## Getting Started\n### API Reference ###\n";

    let headings = extract_headings(markdown);

    assert_eq!(headings.len(), 3);
    assert_eq!(headings[0].level, 1);
    assert_eq!(headings[0].text, "Intro");
    assert_eq!(headings[0].line, 1);
    assert_eq!(headings[0].anchor, "intro");
    assert_eq!(headings[2].text, "API Reference");
    assert_eq!(headings[2].anchor, "api-reference");
}

#[test]
fn extracts_links_without_counting_images() {
    let markdown = "See [Rust](https://www.rust-lang.org/ \"Rust\") and ![Logo](logo.png).";

    let links = extract_links(markdown);

    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "Rust");
    assert_eq!(links[0].target, "https://www.rust-lang.org/");
    assert_eq!(links[0].title.as_deref(), Some("Rust"));
    assert_eq!(links[0].line, 1);
}

#[test]
fn extracts_images() {
    let markdown = "![Diagram](./diagram.svg 'Architecture')";

    let images = extract_images(markdown);

    assert_eq!(images.len(), 1);
    assert_eq!(images[0].alt, "Diagram");
    assert_eq!(images[0].source, "./diagram.svg");
    assert_eq!(images[0].title.as_deref(), Some("Architecture"));
    assert_eq!(images[0].line, 1);
}

#[test]
fn extracts_code_fences_with_language_and_content() {
    let markdown = "~~~rust\nfn main() {}\n~~~\n\n```text\nalpha\nbeta\n```";

    let fences = extract_code_fences(markdown);

    assert_eq!(fences.len(), 2);
    assert_eq!(fences[0].language.as_deref(), Some("rust"));
    assert_eq!(fences[0].content, "fn main() {}");
    assert_eq!(fences[0].start_line, 1);
    assert_eq!(fences[0].end_line, 3);
    assert_eq!(fences[1].language.as_deref(), Some("text"));
    assert_eq!(fences[1].content, "alpha\nbeta");
}

#[test]
fn ignores_headings_inside_code_fences() {
    let markdown = "# Visible\n```md\n## Hidden\n```\n## Also Visible";

    let headings = extract_headings(markdown);
    let texts: Vec<&str> = headings
        .iter()
        .map(|heading| heading.text.as_str())
        .collect();

    assert_eq!(texts, vec!["Visible", "Also Visible"]);
}

#[test]
fn extracts_and_detects_frontmatter() {
    let markdown = "---\ntitle: Example\ntags:\n  - rust\n---\n# Heading\n";

    assert!(has_frontmatter(markdown));
    assert_eq!(
        extract_frontmatter(markdown),
        Some("title: Example\ntags:\n  - rust")
    );
}

#[test]
fn strips_frontmatter() {
    let markdown = "+++\ntitle = \"Example\"\n+++\n# Heading\n";

    assert_eq!(strip_frontmatter(markdown), "# Heading\n");
}

#[test]
fn converts_markdown_to_plain_text() {
    let markdown = "---\ntitle: Demo\n---\n# Hello *World*\n\n> quoted [link](https://example.com)\n- Item one\n1. Item two\n![Logo](logo.png)\n\n```rust\nfn main() {}\n```\n---\n";

    assert_eq!(
        markdown_to_plain_text(markdown),
        "Hello World\nquoted link\nItem one\nItem two\nLogo\nfn main() {}"
    );
}

#[test]
fn extracts_outline_from_headings() {
    let markdown = "# Intro\n## Setup\n## Usage\n### Advanced";
    let outline = extract_outline(markdown);

    assert_eq!(outline.headings.len(), 4);
    assert_eq!(outline.headings[3].anchor, "advanced");
}

#[test]
fn generates_heading_anchors() {
    assert_eq!(heading_to_anchor("Hello, World!"), "hello-world");
    assert_eq!(
        heading_to_anchor("Rust's Markdown Guide"),
        "rusts-markdown-guide"
    );
    assert_eq!(heading_to_anchor("  Already--Spaced  "), "already-spaced");
}

#[test]
fn detects_list_items() {
    assert!(is_unordered_list_item("- first item"));
    assert!(is_unordered_list_item("  * nested item"));
    assert!(is_ordered_list_item("1. first"));
    assert!(is_ordered_list_item("42) second"));
    assert!(!is_unordered_list_item("---"));
    assert!(!is_ordered_list_item("1.no-space"));
}

#[test]
fn detects_blockquotes() {
    assert!(is_blockquote("> quote"));
    assert!(is_blockquote("   > nested"));
    assert!(!is_blockquote("quote"));
}

#[test]
fn detects_horizontal_rules() {
    assert!(is_horizontal_rule("---"));
    assert!(is_horizontal_rule("* * *"));
    assert!(is_horizontal_rule("_ _ _ _"));
    assert!(!is_horizontal_rule("--"));
    assert!(!is_horizontal_rule("- item"));
}

#[test]
fn handles_empty_input() {
    assert!(extract_headings("").is_empty());
    assert!(extract_links("").is_empty());
    assert!(extract_images("").is_empty());
    assert!(extract_code_fences("").is_empty());
    assert_eq!(markdown_to_plain_text(""), "");
    assert_eq!(extract_frontmatter(""), None);
}

#[test]
fn handles_malformed_markdown_without_panicking() {
    let markdown = "# Heading\n[broken link](\n![image]\n```rust\n# not a heading\n";

    assert_eq!(extract_headings(markdown).len(), 1);
    assert!(extract_links(markdown).is_empty());
    assert!(extract_images(markdown).is_empty());

    let fences = extract_code_fences(markdown);
    assert_eq!(fences.len(), 1);
    assert_eq!(fences[0].end_line, markdown.lines().count());
}
