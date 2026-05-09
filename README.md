# use-text

Composable text primitives for Rust.

`use-text` provides small utilities for case conversion, slug generation, truncation, whitespace normalization, and basic readability metrics.

`use-text` is a sibling set to `use-math`, `use-color`, and `use-wave`. Crates stay one layer deep so each crate remains independently useful and easy to compose.

## Workspace crates

- [`use-case`](./crates/use-case): ASCII-oriented case conversion helpers.
- [`use-slug`](./crates/use-slug): URL-safe slug generation.
- [`use-truncate`](./crates/use-truncate): Character- and word-aware truncation helpers.
- [`use-normalize`](./crates/use-normalize): Whitespace and line normalization helpers.
- [`use-readable`](./crates/use-readable): Readability and text metric helpers.

## Examples

```rust
use use_case::{to_kebab_case, to_snake_case};
use use_normalize::normalize_whitespace;
use use_readable::{estimated_reading_time_minutes, word_count};
use use_slug::slugify;
use use_truncate::truncate_with_ellipsis;

let name = to_snake_case("HTTP Server Config");
let route = slugify("RustUse: Composable Text Primitives!");
let preview = truncate_with_ellipsis("Composable text primitives for Rust.", 20);
let clean = normalize_whitespace("  composable   text\n\nprimitives  ");
let words = word_count(&clean);
let minutes = estimated_reading_time_minutes(&clean);

assert_eq!(name, "http_server_config");
assert_eq!(to_kebab_case("hello world"), "hello-world");
assert_eq!(route, "rustuse-composable-text-primitives");
assert_eq!(preview, "Composable text pri…");
assert_eq!(clean, "composable text primitives");
assert_eq!(words, 3);
assert_eq!(minutes, 1);
```

## Deferred future crates

- `use-markdown-text`
- `use-readable-score`
- `use-token-count`
- `use-word-wrap`
- `use-diff-text`
- `use-template-text`
