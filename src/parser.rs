use crate::scanner::Token;
use std::any::Any;
enum Expr<'a> {
    Binary((Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>)),
    Grouping(Box<Expr<'a>>),
    Literal(Box<dyn Any>),
    Unary((Token<'a>, Box<Expr<'a>>)),
}
