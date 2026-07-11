use crate::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr)
}
