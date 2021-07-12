use crate::token::*;
use std::fmt;

pub enum Node {
    Binary(BiNode),
    Unary(UniNode),
    Number(Token),
}

pub struct BiNode {
    pub left: Box<Node>,
    pub op: Token,
    pub right: Box<Node>,
}

pub struct UniNode {
    pub op: Token,
    pub node: Box<Node>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Binary(node) => write!(
                f,
                "Binary {{ left: {:?}, op: {:?}, right: {:?} }}",
                node.left, node.op.token_type, node.right
            ),
            Node::Unary(node) => write!(
                f,
                "Unary {{ op: {:?}, node: {:?} }}",
                node.op.token_type, node.node
            ),
            Node::Number(node) => write!(f, "{:?}", node.literal),
        }
    }
}
