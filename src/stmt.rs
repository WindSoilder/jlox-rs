use crate::{Expr, Token};

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl VarDecl {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        Self { name, initializer }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

impl While {
    pub fn new(condition: Expr, body: Box<Stmt>) -> Self {
        Self { condition, body }
    }
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl FuncDecl {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Self { name, params, body }
    }
}

#[derive(Debug, Clone)]
pub struct Return {
    pub keyword: Token,
    pub value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    If(If),
    While(While),
    Print(Expr),
    Var(VarDecl),
    Func(FuncDecl),
    Block(Block),
    Return(Return),
}
