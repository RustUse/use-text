# use-word

Composable word-level text primitives for RustUse.

`use-word` builds on conservative word boundaries and normalized lowercase comparisons. It is meant
for practical text inspection, not stemming, language modeling, or linguistic analysis.

## Included primitives

- `word_count`
- `unique_words`
- `normalize_word`
- `contains_word`
- `starts_with_word`
- `ends_with_word`
- `words`

## Example

```rust
use use_word::{contains_word, unique_words, word_count};

assert_eq!(word_count("Hello, hello world"), 3);
assert!(contains_word("Hello, world", "world"));
assert_eq!(unique_words("Hello, hello world").len(), 2);
```
