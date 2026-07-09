use crate::Expr;

pub enum Stmt {
    Expression(Expr),
    Print(Expr)
}
