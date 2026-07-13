use crate::{Literal, Token};
#[derive(Debug, Clone)]
pub enum Expr {
    Binary((Box<Expr>, Token, Box<Expr>)),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary((Token, Box<Expr>)),
    Var(Token),
    Garbage,
}
