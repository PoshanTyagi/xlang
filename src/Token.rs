#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
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
    STRING,
    INT,
    FLOAT,

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
