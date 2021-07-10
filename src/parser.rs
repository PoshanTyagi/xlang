// use crate::helper::*;
use crate::node::*;
use crate::token::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Result<Parser, String> {
        let local_tokens = tokens.clone();
        let token = local_tokens.get(0);
        if token.is_none() {
            return Err(String::from("Something went wrong"));
        }

        let token = token.unwrap().clone();

        Ok(Parser {
            current_token: token,
            tokens: local_tokens,
            pos: 0,
        })
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.tokens.len() {
            let token = self.tokens.get(self.pos);
            if token.is_none() {
                println!("Something is wrong");
            }
            self.current_token = token.unwrap().clone();
        } else {
            self.current_token = Token::new(TokenType::NIL, Literal::NIL);
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_token.token_type == TokenType::EOF
    }

    fn match_tokens(&self, types: &[TokenType]) -> bool {
        for i in 0..types.len() {
            if self.check(&types[i]) {
                return true;
            }
        }
        return false;
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        *token_type == self.current_token.token_type
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        return self.expression();
    }

    fn expression(&mut self) -> Result<Node, String> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Node, String> {
        let left = self.comparison();

        let mut left_value = match left {
            Ok(left) => left,
            Err(left) => return Err(left),
        };

        while self.match_tokens(&[TokenType::NOT_EQUAL, TokenType::EQUAL_EQUAL]) {
            let op_tok = self.current_token.clone();
            self.advance();
            let right = self.comparison();

            let right_value = match right {
                Ok(right) => right,
                Err(right) => return Err(right),
            };

            left_value = Node::BiNode {
                left: Box::new(left_value),
                op: op_tok,
                right: Box::new(right_value),
            };
        }

        return Ok(left_value);
    }

    fn comparison(&mut self) -> Result<Node, String> {
        let left = self.term();

        let mut left_value = match left {
            Ok(left) => left,
            Err(left) => return Err(left),
        };

        while self.match_tokens(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let op_tok = self.current_token.clone();
            self.advance();
            let right = self.term();

            let right_value = match right {
                Ok(right) => right,
                Err(right) => return Err(right),
            };

            left_value = Node::BiNode {
                left: Box::new(left_value),
                op: op_tok,
                right: Box::new(right_value),
            };
        }

        return Ok(left_value);
    }

    fn term(&mut self) -> Result<Node, String> {
        let left = self.factor();

        let mut left_value = match left {
            Ok(left) => left,
            Err(left) => return Err(left),
        };

        while self.match_tokens(&[TokenType::MINUS, TokenType::PLUS]) {
            let op_tok = self.current_token.clone();
            self.advance();
            let right = self.factor();

            let right_value = match right {
                Ok(right) => right,
                Err(right) => return Err(right),
            };

            left_value = Node::BiNode {
                left: Box::new(left_value),
                op: op_tok,
                right: Box::new(right_value),
            };
        }

        return Ok(left_value);
    }

    fn factor(&mut self) -> Result<Node, String> {
        let left = self.unary();

        let mut left_value = match left {
            Ok(left) => left,
            Err(left) => return Err(left),
        };

        while self.match_tokens(&[TokenType::SLASH, TokenType::STAR]) {
            let op_tok = self.current_token.clone();
            self.advance();
            let right = self.unary();

            let right_value = match right {
                Ok(right) => right,
                Err(right) => return Err(right),
            };

            left_value = Node::BiNode {
                left: Box::new(left_value),
                op: op_tok,
                right: Box::new(right_value),
            };
        }

        return Ok(left_value);
    }

    fn unary(&mut self) -> Result<Node, String> {
        if self.match_tokens(&[TokenType::MINUS, TokenType::PLUS]) {
            let op_tok = self.current_token.clone();
            self.advance();
            let right = self.unary();

            let right_value = match right {
                Ok(right) => right,
                Err(right) => return Err(right),
            };

            return Ok(Node::UniNode {
                op: op_tok,
                node: Box::new(right_value),
            });
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Node, String> {
        if self.match_tokens(&[TokenType::INT, TokenType::FLOAT]) {
            let tok = self.current_token.clone();
            self.advance();
            return Ok(Node::Number(tok));
        } else if self.match_tokens(&[TokenType::LEFT_PAREN]) {
            self.advance();
            let expr = self.expression();

            let expr_value = match expr {
                Ok(expr) => expr,
                Err(expr) => return Err(expr),
            };

            if self.match_tokens(&[TokenType::RIGHT_PAREN]) {
                self.advance();
                return Ok(expr_value);
            } else {
                return Err(String::from("Syntax Error"));
            }
        } else {
            return Err(String::from("Syntax Error"));
        }
    }
}
