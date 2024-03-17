use Token::*;

#[derive(Debug)]
pub enum Token {
    Digit(char),
    Plus,
    LParen,
    RParen,
}

#[derive(Debug)]
pub struct SyntaxError {
    message: String,
}

impl SyntaxError {
    fn new(message: String) -> SyntaxError {
        SyntaxError { message }
    }
}

pub fn tokenize(string: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = string.chars();
    while let Some(ch) = iter.next() {
        let tok = match ch {
            '0'..='9' => Digit(ch),
            '+' => Plus,
            '(' => LParen,
            ')' => RParen,
            ch if ch.is_whitespace() => continue,
            _ => return Err(SyntaxError::new(format!("Unexpected character '{}'", ch))),
        };
        tokens.push(tok)
    }
    Ok(tokens)
}
