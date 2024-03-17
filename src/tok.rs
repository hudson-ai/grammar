use Token::*;

#[derive(Debug)]
pub enum Token {
    Digit(char),
    Plus,
    LParen,
    RParen,
}

pub fn tokenize(string: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = string.chars();
    while let Some(ch) = iter.next() {
        let tok = match ch {
            '0'..='9' => Digit(ch),
            '+' => Plus,
            '(' => LParen,
            ')' => RParen,
            ch if ch.is_whitespace() => continue,
            _ => panic!("Unexpected character '{}'", ch),
        };
        tokens.push(tok)
    };
    tokens
}

