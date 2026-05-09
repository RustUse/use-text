#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A token with its kind and byte span.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    /// The token classification.
    pub kind: TokenKind,
    /// The owned token text.
    pub text: String,
    /// The token span in the original input.
    pub span: TokenSpan,
}

impl Token {
    fn new(kind: TokenKind, text: String, start: usize, end: usize) -> Self {
        Self {
            kind,
            text,
            span: TokenSpan { start, end },
        }
    }
}

/// The category assigned to a token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
    /// A non-whitespace segment split by whitespace.
    Text,
    /// A conservative word token.
    Word,
    /// A conservative sentence token.
    Sentence,
    /// A single Unicode scalar value.
    Char,
}

/// A byte span in the original input string.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenSpan {
    /// Inclusive start byte offset.
    pub start: usize,
    /// Exclusive end byte offset.
    pub end: usize,
}

/// Small configuration for future tokenizer extensions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenizerOptions {
    /// Whether empty tokens should be dropped.
    pub trim_empty: bool,
    /// Whether surrounding whitespace should be kept in higher-level flows.
    pub include_whitespace: bool,
}

impl Default for TokenizerOptions {
    fn default() -> Self {
        Self {
            trim_empty: true,
            include_whitespace: false,
        }
    }
}

/// Splits input on contiguous whitespace.
pub fn tokenize_whitespace(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut start = None;

    for (index, character) in input.char_indices() {
        if character.is_whitespace() {
            if let Some(token_start) = start.take() {
                tokens.push(Token::new(
                    TokenKind::Text,
                    input[token_start..index].to_owned(),
                    token_start,
                    index,
                ));
            }
        } else if start.is_none() {
            start = Some(index);
        }
    }

    if let Some(token_start) = start {
        tokens.push(Token::new(
            TokenKind::Text,
            input[token_start..].to_owned(),
            token_start,
            input.len(),
        ));
    }

    tokens
}

/// Extracts conservative word tokens.
pub fn tokenize_words(input: &str) -> Vec<Token> {
    word_ranges(input)
        .into_iter()
        .map(|(start, end)| Token::new(TokenKind::Word, input[start..end].to_owned(), start, end))
        .collect()
}

/// Extracts conservative sentence tokens.
pub fn tokenize_sentences(input: &str) -> Vec<Token> {
    let characters: Vec<(usize, char)> = input.char_indices().collect();
    let mut tokens = Vec::new();
    let mut start = None;
    let mut last_non_whitespace_end = 0;
    let mut index = 0;

    while index < characters.len() {
        let (byte_index, character) = characters[index];
        let character_end = byte_index + character.len_utf8();

        if start.is_none() {
            if character.is_whitespace() {
                index += 1;
                continue;
            }

            start = Some(byte_index);
        }

        if !character.is_whitespace() {
            last_non_whitespace_end = character_end;
        }

        if matches!(character, '.' | '!' | '?') {
            let mut sentence_end = character_end;
            let mut lookahead = index + 1;

            while let Some((next_byte, next_character)) = characters.get(lookahead).copied() {
                if matches!(
                    next_character,
                    '.' | '!' | '?' | '"' | '\'' | '”' | '’' | ')' | ']'
                ) {
                    sentence_end = next_byte + next_character.len_utf8();
                    lookahead += 1;
                } else {
                    break;
                }
            }

            let next_character = characters.get(lookahead).map(|(_, value)| *value);
            if next_character.is_none() || next_character.is_some_and(char::is_whitespace) {
                let token_start = start.expect("sentence start should exist");
                tokens.push(Token::new(
                    TokenKind::Sentence,
                    input[token_start..sentence_end].to_owned(),
                    token_start,
                    sentence_end,
                ));
                start = None;
                last_non_whitespace_end = sentence_end;
                index = lookahead;
                continue;
            }
        }

        index += 1;
    }

    if let Some(token_start) = start {
        tokens.push(Token::new(
            TokenKind::Sentence,
            input[token_start..last_non_whitespace_end].to_owned(),
            token_start,
            last_non_whitespace_end,
        ));
    }

    tokens
}

/// Splits input into Unicode scalar values.
pub fn tokenize_chars(input: &str) -> Vec<Token> {
    input
        .char_indices()
        .map(|(start, character)| {
            let end = start + character.len_utf8();
            Token::new(TokenKind::Char, character.to_string(), start, end)
        })
        .collect()
}

/// Counts conservative word tokens.
pub fn token_count(input: &str) -> usize {
    tokenize_words(input).len()
}

fn word_ranges(input: &str) -> Vec<(usize, usize)> {
    let characters: Vec<(usize, char)> = input.char_indices().collect();
    let mut ranges = Vec::new();
    let mut start = None;

    for (index, (byte_index, character)) in characters.iter().copied().enumerate() {
        let previous = index.checked_sub(1).map(|value| characters[value].1);
        let next = characters.get(index + 1).map(|(_, value)| *value);
        let is_word_character =
            character.is_alphanumeric() || is_apostrophe(previous, character, next);

        if is_word_character {
            if start.is_none() {
                start = Some(byte_index);
            }
        } else if let Some(token_start) = start.take() {
            ranges.push((token_start, byte_index));
        }
    }

    if let Some(token_start) = start {
        ranges.push((token_start, input.len()));
    }

    ranges
}

fn is_apostrophe(previous: Option<char>, current: char, next: Option<char>) -> bool {
    matches!(current, '\'' | '’')
        && previous.is_some_and(char::is_alphanumeric)
        && next.is_some_and(char::is_alphanumeric)
}

#[cfg(test)]
mod tests {
    use super::{
        TokenKind, TokenizerOptions, token_count, tokenize_chars, tokenize_sentences,
        tokenize_whitespace, tokenize_words,
    };

    #[test]
    fn handles_empty_and_whitespace_only_input() {
        assert!(tokenize_whitespace("").is_empty());
        assert!(tokenize_words("   \n").is_empty());
        assert_eq!(token_count("\t  "), 0);
    }

    #[test]
    fn tokenizes_whitespace_and_tracks_spans() {
        let tokens = tokenize_whitespace(" hello  world ");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Text);
        assert_eq!(tokens[0].text, "hello");
        assert_eq!(tokens[0].span.start, 1);
        assert_eq!(tokens[1].span.end, 13);
    }

    #[test]
    fn tokenizes_words_with_punctuation_and_apostrophes() {
        let tokens = tokenize_words("Hello, world! don't-stop");
        let texts: Vec<&str> = tokens.iter().map(|token| token.text.as_str()).collect();
        assert_eq!(texts, vec!["Hello", "world", "don't", "stop"]);
        assert!(tokens.iter().all(|token| token.kind == TokenKind::Word));
    }

    #[test]
    fn tokenizes_sentences_and_multiline_text() {
        let tokens = tokenize_sentences("One.  Two!\nThree");
        let texts: Vec<&str> = tokens.iter().map(|token| token.text.as_str()).collect();
        assert_eq!(texts, vec!["One.", "Two!", "Three"]);
    }

    #[test]
    fn tokenizes_unicode_characters() {
        let tokens = tokenize_chars("A🙂");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[1].text, "🙂");
        assert_eq!(tokens[1].span.start, 1);
        assert_eq!(tokens[1].span.end, 5);
    }

    #[test]
    fn tokenizes_unicode_words_conservatively() {
        let tokens = tokenize_words("naïve façade");
        let texts: Vec<&str> = tokens.iter().map(|token| token.text.as_str()).collect();
        assert_eq!(texts, vec!["naïve", "façade"]);
        assert!(TokenizerOptions::default().trim_empty);
    }
}
