use crate::{Literal, Token};
#[derive(Debug, Clone)]
pub enum Expr {
    Binary((Box<Expr>, Token, Box<Expr>)),
    Logical((Box<Expr>, Token, Box<Expr>)),
    /// callee, paren, arguments
    /// it stores the token for the closing parenthesis, we'll
    /// use that token's location when we report a runtime error caused by
    /// a function call.
    Call((Box<Expr>, Token, Vec<Expr>)),
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
