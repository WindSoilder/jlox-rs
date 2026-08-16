// TODO: remove it, allow it for now to reduce IDE noise.
#![allow(unused)]

use crate::{Block, Expr, FuncDecl, Stmt, Token, error_at_token};
use std::collections::HashMap;
use std::ptr::addr_of;

use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FunctionType {
    None,
    Function,
}

pub struct Resolver {
    /// This field keep track of the stack of scopes currently, in scope.
    /// Each element is a HashMap representing a single block scope.
    /// Keys are variable names.
    scopes: Vec<HashMap<String, bool>>,
    results: HashMap<*const Expr, usize>,
    current_function: FunctionType,
    catch_error: bool,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scopes: vec![],
            results: HashMap::new(),
            current_function: FunctionType::None,
            catch_error: false,
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) {
        for one_stmt in statements {
            self.resolve_stmt(one_stmt)
        }
    }

    fn resolve_block(&mut self, block: &Block) {
        self.begin_scope();
        self.resolve(&block.statements);
        self.end_scope();
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_stmt(&mut self, statement: &Stmt) {
        match statement {
            Stmt::Var(var_decl) => {
                self.declare(&var_decl.name);
                if let Some(initializer) = &var_decl.initializer {
                    self.resolve_expr(initializer)
                }
                self.define(&var_decl.name);
            }
            Stmt::Func(func_decl) => {
                self.declare(&func_decl.name);
                self.define(&func_decl.name);

                self.resolve_function(func_decl, FunctionType::Function);
            }
            Stmt::Expression(expr) => self.resolve_expr(expr),
            Stmt::If(if_stmt) => {
                self.resolve_expr(&if_stmt.condition);
                self.resolve_stmt(&if_stmt.then_branch);
                if let Some(else_branch) = &if_stmt.else_branch {
                    self.resolve_stmt(else_branch);
                }
            }
            Stmt::Print(print_stmt) => {
                self.resolve_expr(print_stmt);
            }
            Stmt::Return(return_stmt) => {
                if self.current_function == FunctionType::None {
                    self.catch_error = true;
                    error_at_token(&return_stmt.keyword, "Can't return from top-level code.");
                }
                if let Some(return_val) = &return_stmt.value {
                    self.resolve_expr(return_val);
                }
            }
            Stmt::While(while_stmt) => {
                self.resolve_expr(&while_stmt.condition);
                self.resolve_stmt(&while_stmt.body);
            }
            Stmt::Block(block) => {
                self.resolve_block(block);
            }
        }
    }

    fn resolve_function(&mut self, func_decl: &FuncDecl, func_type: FunctionType) {
        self.begin_scope();
        let enclosing_function = self.current_function;
        self.current_function = func_type;
        for param in func_decl.params.iter() {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&func_decl.body);
        self.end_scope();
        self.current_function = enclosing_function;
    }

    fn declare(&mut self, name: &Token) {
        let length = self.scopes.len();
        if !self.scopes.is_empty() {
            let current_scope = &mut self.scopes[length - 1];
            if current_scope.contains_key(&name.lexeme) {
                self.catch_error = true;
                error_at_token(name, "Already a variable with this name in this scope.")
            } else {
                self.scopes[length - 1].insert(name.lexeme.clone(), false);
            }
        }
    }

    fn define(&mut self, name: &Token) {
        let length = self.scopes.len();
        if !self.scopes.is_empty() {
            self.scopes[length - 1].insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Var(var) => {
                if !self.scopes.is_empty()
                    && self.scopes[self.scopes.len() - 1].get(&var.lexeme) == Some(&false)
                {
                    self.catch_error = true;
                    error_at_token(var, "Can't read local variable in its own initializer.");
                }
                // The variable `var` is connected with `expr`.
                self.resolve_local(expr, &var);
            }
            Expr::Binary((left, _, right)) => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Call((callee, _, arguments)) => {
                self.resolve_expr(callee);

                for arg in arguments {
                    self.resolve_expr(arg);
                }
            }
            Expr::Grouping(expr) => {
                self.resolve_expr(expr);
            }
            Expr::Logical((left, _, right)) => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Unary((_, right)) => {
                self.resolve_expr(right);
            }
            Expr::Assignment((name, value)) => {
                self.resolve_expr(value);
                self.resolve_local(expr, name);
            }
            Expr::Literal(_) => (),
            Expr::Garbage => (),
        }
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                self.results
                    .insert(expr as *const Expr, self.scopes.len() - 1 - i);
                break;
            }
        }
    }

    pub fn output_locals(self) -> (HashMap<*const Expr, usize>, bool) {
        (self.results, self.catch_error)
    }
}
