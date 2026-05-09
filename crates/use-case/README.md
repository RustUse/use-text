# use-case

Composable string casing primitives for RustUse.

`use-case` keeps case conversion predictable and lightweight. It is aimed at repository naming,
identifier shaping, and small formatting tasks instead of locale-aware typography.

## Included conversions

- `to_snake_case`
- `to_kebab_case`
- `to_pascal_case`
- `to_camel_case`
- `to_title_case`
- `to_constant_case`
- `detect_case`

## Example

```rust
use use_case::{detect_case, to_kebab_case, to_pascal_case, TextCase};

assert_eq!(to_kebab_case("HTTPServerError"), "http-server-error");
assert_eq!(to_pascal_case("user_profile"), "UserProfile");
assert_eq!(detect_case("userProfile"), TextCase::Camel);
```

## Scope

- ASCII-first and predictable.
- Conservative with punctuation and repeated separators.
- Unicode letters are preserved for case conversion where Rust's built-in casing supports them.