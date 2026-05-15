# use-markdown

Composable Markdown text primitives for RustUse.

`use-markdown` provides lightweight helpers for inspecting headings, links, images, code fences,
frontmatter, outlines, and plain text in Markdown documents. It is intentionally small and does
not try to be a full CommonMark parser or HTML renderer.

## Experimental status

This crate is experimental before `0.3.0`. Expect small API adjustments while the focused surface
settles.

## Example

```rust
use use_markdown::{extract_headings, extract_links, markdown_to_plain_text};

let markdown = "# Hello World\n\nSee [Rust](https://www.rust-lang.org/).";

let headings = extract_headings(markdown);
assert_eq!(headings[0].anchor, "hello-world");

let links = extract_links(markdown);
assert_eq!(links[0].target, "https://www.rust-lang.org/");

assert_eq!(markdown_to_plain_text(markdown), "Hello World\nSee Rust.");
```

## Scope

- ATX heading extraction
- Inline link and image extraction
- Fenced code block extraction
- Frontmatter detection and stripping
- Markdown-to-plain-text cleanup for practical tooling
- Lightweight outline generation and heading anchors
- Small line-based checks for lists, blockquotes, and horizontal rules

## Non-goals

- Full CommonMark compliance
- A Markdown AST
- HTML rendering
- Reference-style link resolution
- Heavy parser dependencies

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
