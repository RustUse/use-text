# use-case

ASCII-oriented text case conversion helpers.

`use-case` normalizes separator-delimited and mixed-case ASCII inputs into snake, kebab, camel, pascal, and title case.

## Unicode behavior

Word boundary detection is ASCII-oriented for v0.1. Non-ASCII letters are kept as part of words, but acronym splitting rules target ASCII case transitions.
