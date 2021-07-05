use crate::helper::*;
use crate::token::*;

fn keywords(text: String) -> Option<TokenType> {
    match text.as_str() {
        "and" => Some(TokenType::AND),
        "else" => Some(TokenType::ELSE),
        "false" => Some(TokenType::FALSE),
        "fun" => Some(TokenType::FUN),
        "for" => Some(TokenType::FOR),
        "if" => Some(TokenType::IF),
        "nil" => Some(TokenType::NIL),
        "or" => Some(TokenType::OR),
        "print" => Some(TokenType::PRINT),
        "return" => Some(TokenType::RETURN),
        "true" => Some(TokenType::TRUE),
        "var" => Some(TokenType::VAR),
        "while" => Some(TokenType::WHILE),
        _ => None,
    }
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: i64,
    current: i64,
    line: i64,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            let result = self.get_token();
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            String::new(),
            Literal::NIL,
            self.line,
        ));
        return Ok(self.tokens.clone());
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i64
    }

    fn advance(&mut self) -> char {
        let c = self.source.char_at(self.current);
        self.current += 1;
        return c;
    }

    fn get_current(&mut self) -> char {
        let c = self.source.char_at(self.current);
        return c;
    }

    fn get_token(&mut self) -> Result<(), String> {
        let c = self.get_current();
        match c {
            ' ' | '\n' | '\r' => {
                self.advance();
            }
            '=' => {
                self.advance();
                if self.get_current() == '=' {
                    self.advance();
                    self.add_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            }
            '>' => {
                self.advance();
                if self.get_current() == '=' {
                    self.advance();
                    self.add_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }
            '<' => {
                self.advance();
                if self.get_current() == '=' {
                    self.advance();
                    self.add_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_token(TokenType::LESS)
                }
            }
            '!' => {
                self.advance();
                if self.get_current() == '=' {
                    self.advance();
                    self.add_token(TokenType::NOT_EQUAL)
                } else {
                    self.add_token(TokenType::NOT)
                }
            }
            '(' => {
                self.advance();
                self.add_token(TokenType::LEFT_PAREN)
            }
            ')' => {
                self.advance();
                self.add_token(TokenType::RIGHT_PAREN)
            }
            '{' => {
                self.advance();
                self.add_token(TokenType::LEFT_BRACE)
            }
            '}' => {
                self.advance();
                self.add_token(TokenType::RIGHT_BRACE)
            }
            ',' => {
                self.advance();
                self.add_token(TokenType::COMMA)
            }
            '.' => {
                self.advance();
                self.add_token(TokenType::DOT)
            }
            '-' => {
                self.advance();
                self.add_token(TokenType::MINUS)
            }
            '+' => {
                self.advance();
                self.add_token(TokenType::PLUS)
            }
            ';' => {
                self.advance();
                self.add_token(TokenType::SEMICOLON)
            }
            '/' => {
                self.advance();
                self.add_token(TokenType::SLASH)
            }
            '*' => {
                self.advance();
                self.add_token(TokenType::STAR)
            }
            '"' => {
                let result = self.string();
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
            }
            c if c.is_numeric() => {
                let result = self.number();
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
            }
            c if c.is_alphabetic() => {
                let result = self.identifier();
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
            }
            _ => {
                self.advance();
                return Err(String::from("Unexpected tokens"));
            }
        }
        return Ok(());
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::NIL);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens.push(Token::new(
            token_type,
            self.source.substring(self.start, self.current),
            literal,
            self.line,
        ));
    }

    fn number(&mut self) -> Result<(), String> {
        let mut temp_number = String::new();
        let mut dot_count = 0;

        while self.get_current().is_numeric() || self.get_current() == '.' {
            if self.get_current() == '.' {
                if dot_count == 1 {
                    return Err(String::from("To many decimals"));
                }
                dot_count += 1;
            }
            temp_number.push(self.get_current());
            self.advance();
        }

        if dot_count == 0 {
            self.add_token_with_literal(
                TokenType::INT,
                Literal::INT(temp_number.parse::<i64>().unwrap()),
            )
        } else if dot_count == 1 {
            self.add_token_with_literal(
                TokenType::FLOAT,
                Literal::FLOAT(temp_number.parse::<f64>().unwrap()),
            )
        }
        return Ok(());
    }

    fn identifier(&mut self) -> Result<(), String> {
        let mut temp = String::new();
        while self.get_current().is_alphanumeric() {
            temp.push(self.get_current());
            self.advance();
        }

        let keyword = keywords(temp.clone());
        match keyword {
            Some(keyword) => self.add_token(keyword),
            None => self.add_token_with_literal(TokenType::IDENTIFIER, Literal::STR(temp)),
        }
        return Ok(());
    }
    fn string(&mut self) -> Result<(), String> {
        let mut temp = String::new();
        self.advance();
        while !self.is_at_end() && self.get_current() != '"' {
            temp.push(self.get_current());
            self.advance();
        }
        if self.get_current() == '"' {
            self.advance();
            self.add_token_with_literal(TokenType::STRING, Literal::STR(temp));
        } else {
            return Err(String::from("'\"' is missing"));
        }
        return Ok(());
    }
}
