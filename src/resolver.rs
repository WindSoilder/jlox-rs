// TODO: remove it, allow it for now to reduce IDE noise.
#![allow(unused)]

use crate::{Block, Expr, Stmt, Token, error_at_token};
use std::collections::HashMap;

use anyhow::Result;

struct Resolver {
    /// This field keep track of the stack of scopes currently, in scope.
    /// Each element is a HashMap representing a single block scope.
    /// Keys are variable names.
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<()> {
        for one_stmt in statements {
            self.resolve_stmt(one_stmt)?
        }
        Ok(())
    }

    fn resolve_block(&mut self, block: &Block) -> Result<()> {
        self.begin_scope();
        self.resolve(&block.statements)?;
        self.end_scope();
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_stmt(&mut self, statement: &Stmt) -> Result<()> {
        match statement {
            Stmt::Var(var_decl) => {
                self.declare(&var_decl.name)?;
                if let Some(initializer) = &var_decl.initializer {
                    self.resolve_expr(initializer)?
                }
                self.define(&var_decl.name)?;
                Ok(())
            }
            _ => todo!(),
        }
    }

    fn declare(&mut self, name: &Token) -> Result<()> {
        let length = self.scopes.len();
        if !self.scopes.is_empty() {
            self.scopes[length - 1].insert(name.lexeme.clone(), false);
        }
        Ok(())
    }

    fn define(&mut self, name: &Token) -> Result<()> {
        let length = self.scopes.len();
        if !self.scopes.is_empty() {
            self.scopes[length - 1].insert(name.lexeme.clone(), true);
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Var(var) => {
                if !self.scopes.is_empty()
                    && self.scopes[self.scopes.len() - 1].get(&var.lexeme) == Some(&false)
                {
                    error_at_token(var, "Can't read local variable in its own initializer.");
                }
                self.resolve_local(expr, &var);
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) -> Result<()> {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
                break
            }
        }
        Ok(())
    }
}
