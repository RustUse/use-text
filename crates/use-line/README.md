# use-line

Composable line-level text primitives for RustUse.

`use-line` provides small helpers for line counting, indentation, dedentation, and line-ending
normalization. It stays explicit about logical lines and keeps the behavior predictable across
LF, CRLF, and CR inputs.

## Included primitives

- `line_count`
- `non_empty_line_count`
- `trim_lines`
- `normalize_line_endings`
- `indent_lines`
- `dedent_lines`
- `lines_with_numbers`

## Example

```rust
use use_line::{dedent_lines, line_count, lines_with_numbers, LineEnding};

assert_eq!(line_count("alpha\nbeta\n"), 2);
assert_eq!(dedent_lines("    alpha\n      beta"), "alpha\n  beta");
assert_eq!(lines_with_numbers("alpha")[0].number.get(), 1);
assert_eq!(LineEnding::Crlf.as_str(), "\r\n");
```
