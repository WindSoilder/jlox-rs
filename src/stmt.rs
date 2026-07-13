use crate::{Expr, Token};

#[derive(Debug)]
pub struct VarDecl {
    pub name: Token,
    pub initializer: Option<Expr>
}

impl VarDecl {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        Self {name, initializer}
    }
}

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(VarDecl)
}
