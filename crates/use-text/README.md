# use-text

Composable text primitives for RustUse.

`use-text` is the thin umbrella crate for the RustUse text workspace. Use it when you want the
common prelude and reexports from `use-case`, `use-slug`, `use-token`, `use-word`, and
`use-line` in one dependency.

## Reexports

- `use_case`
- `use_slug`
- `use_token`
- `use_word`
- `use_line`

## Example

```rust
use use_text::prelude::{slugify, to_snake_case, word_count};

assert_eq!(to_snake_case("HelloWorld"), "hello_world");
assert_eq!(slugify("Hello World"), "hello-world");
assert_eq!(word_count("Hello world"), 2);
```
