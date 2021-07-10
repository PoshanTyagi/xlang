#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    NOT,
    NOT_EQUAL,

    // Literals
    IDENTIFIER,
    INT,
    FLOAT,
    STRING,

    // Keywords
    AND,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    TRUE,
    VAR,
    WHILE,

    // End of File
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    STR(String),
    INT(i64),
    FLOAT(f64),
    NIL,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Literal,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Literal) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}
