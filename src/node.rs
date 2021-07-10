use crate::token::*;

#[derive(Debug)]
pub enum Node {
    BiNode {
        left: Box<Node>,
        op: Token,
        right: Box<Node>,
    },
    UniNode {
        op: Token,
        node: Box<Node>,
    },
    Number(Token),
}
