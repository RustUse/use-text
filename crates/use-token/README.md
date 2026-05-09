# use-token

Composable tokenization primitives for RustUse.

`use-token` keeps tokenization explicit and small. It handles whitespace splitting, conservative
word tokenization, lightweight sentence boundaries, and character spans without claiming to be a
full NLP parser.

## Included primitives

- `tokenize_whitespace`
- `tokenize_words`
- `tokenize_sentences`
- `tokenize_chars`
- `token_count`

## Example

```rust
use use_token::{token_count, tokenize_sentences, tokenize_words};

assert_eq!(token_count("Hello, world!"), 2);
assert_eq!(tokenize_words("don't stop").len(), 2);
assert_eq!(tokenize_sentences("One. Two!").len(), 2);
```
