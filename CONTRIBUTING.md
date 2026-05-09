# Contributing

RustUse/use-text is intentionally small at the API level even though it is a multi-crate workspace.
Contributions should keep the public text primitives explicit, typed, and pragmatic.

## Development flow

1. Make the smallest useful change.
2. Add or update tests for any public behavior change.
3. Prefer direct helpers over broad parsing frameworks.
4. Keep README examples aligned with the actual crate APIs.

## Local validation

```sh
cargo fmt --all --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Scope guidance

- `use-text` is the RustUse text primitives set, not a CLI and not a full NLP system.
- Prefer reusable, testable primitives over framework-style abstractions.
- Keep slugging, casing, tokenization, and line handling predictable and well documented.
- Avoid heavy dependencies unless they materially simplify the implementation.
