# use-text

Composable text primitives for RustUse.

`use-text` is the foundational text primitive layer for RustUse. It provides small, focused crates
for casing, Markdown inspection, slugs, tokenization, words, and lines, plus a thin umbrella crate
for projects that want the whole surface in one dependency.

This repository is not a full NLP framework, not a markdown renderer, not a regex replacement
library, and not a localization system. It stays focused on practical text primitives for v0.1.

## Workspace crates

| Crate          | Purpose                                                |
| -------------- | ------------------------------------------------------ |
| `use-text`     | Thin umbrella crate with reexports and a small prelude |
| `use-case`     | String casing primitives                               |
| `use-markdown` | Markdown inspection and extraction primitives          |
| `use-slug`     | Slug and URL-safe text primitives                      |
| `use-token`    | Simple tokenization primitives                         |
| `use-word`     | Word-level text primitives                             |
| `use-text-line` | Line-level text primitives                             |

## Installation

Install the umbrella crate when you want the full workspace surface:

```toml
[dependencies]
use-text = "0.2.0"
```

Or install a focused crate directly:

```toml
[dependencies]
use-case = "0.1.0"
use-markdown = "0.1.0"
use-slug = "0.1.0"
use-text-line = "0.1.0"
```

## Usage

### Casing

```rust
use use_text::prelude::{detect_case, to_snake_case, TextCase};

assert_eq!(to_snake_case("HTTPServerError"), "http_server_error");
assert_eq!(detect_case("userProfile"), TextCase::Camel);
```

### Slugs

```rust
use use_text::prelude::{is_slug, slugify, truncate_slug};

assert_eq!(slugify(" Release Candidate 1 "), "release-candidate-1");
assert!(is_slug("release-candidate-1"));
assert_eq!(truncate_slug("release-candidate-1", 10), "release");
```

### Markdown

```rust
use use_text::prelude::{extract_headings, markdown_to_plain_text};

let markdown = "# Hello World\n\nSee [Rust](https://www.rust-lang.org/).";

assert_eq!(extract_headings(markdown)[0].anchor, "hello-world");
assert_eq!(markdown_to_plain_text(markdown), "Hello World\nSee Rust.");
```

### Tokens

```rust
use use_text::prelude::{token_count, tokenize_sentences};

assert_eq!(token_count("Hello, world!"), 2);
assert_eq!(tokenize_sentences("One. Two!").len(), 2);
```

### Words

```rust
use use_text::prelude::{contains_word, unique_words, word_count};

assert_eq!(word_count("Hello, hello world"), 3);
assert!(contains_word("Hello, world", "world"));
assert_eq!(unique_words("Hello, hello world").len(), 2);
```

### Lines

```rust
use use_text::prelude::{dedent_lines, line_count, LineEnding, normalize_line_endings};

assert_eq!(line_count("alpha\nbeta\n"), 2);
assert_eq!(dedent_lines("    alpha\n      beta"), "alpha\n  beta");
assert_eq!(normalize_line_endings("alpha\r\nbeta", LineEnding::Lf), "alpha\nbeta");
```

## Project status

- Version `0.2.0` moves line-level text helpers to the `use-text-line` package so `use-line` can belong to the geometry set.
- Markdown helpers stay intentionally line-based and lightweight.
- Slug behavior is intentionally ASCII-first and conservative.
- Tokenization is deterministic and lightweight rather than language-aware.
- APIs favor pure functions, `&str` inputs, and owned `String` outputs for transformed text.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
