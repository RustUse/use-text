# use-slug

Composable slug and URL-safe text primitives for RustUse.

`use-slug` is intentionally conservative in v0.1. It lowercases ASCII text, trims whitespace,
collapses repeated separators, and avoids transliteration tables or locale-aware behavior.

## Included primitives

- `slugify`
- `normalize_slug`
- `is_slug`
- `slug_words`
- `truncate_slug`

## Example

```rust
use use_slug::{is_slug, slugify, truncate_slug};

assert_eq!(slugify(" Release Candidate 1 "), "release-candidate-1");
assert!(is_slug("release-candidate-1"));
assert_eq!(truncate_slug("release-candidate-1", 10), "release");
```