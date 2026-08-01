use crate::{Literal, Token};
#[derive(Debug, Clone)]
pub enum Expr {
    Binary((Box<Expr>, Token, Box<Expr>)),
    Logical((Box<Expr>, Token, Box<Expr>)),
    Assignment((Token, Box<Expr>)),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary((Token, Box<Expr>)),
    Var(Token),
    Garbage,
}

impl Expr {
    pub fn is_garbage(&self) -> bool {
        matches!(self, Expr::Garbage)
    }
}
