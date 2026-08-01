use crate::{Expr, Token};

#[derive(Debug)]
pub struct VarDecl {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl VarDecl {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        Self { name, initializer }
    }
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
}

#[derive(Debug)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

impl If {
    pub fn new(condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    If(If),
    Print(Expr),
    Var(VarDecl),
    Block(Block),
}
