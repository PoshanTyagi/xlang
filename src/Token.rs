use crate::TokenType::*;

#[derive(Debug, Clone)]
pub enum Literal {
    STR(String),
    INT(i64),
    FLOAT(f64),
    NIL,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: i64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: i64) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
