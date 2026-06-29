use crate::scanner::Token;
use std::any::Any;
enum Expr {
    Binary((Box<Expr>, Token, Box<Expr>)),
    Grouping(Box<Expr>),
    Literal(Box<dyn Any>),
    Unary((Token, Box<Expr>)),
}
