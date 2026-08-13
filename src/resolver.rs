use std::collections::HashMap;

use crate::{Block, Expr, Stmt, Token, error_at_token};
use anyhow::Result;

struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<()> {
        todo!()
    }

    fn resolve_block(&mut self, block: &Block) -> Result<()> {
        self.begin_scope();
        self.resolve(&block.statements);
        self.end_scope();
        Ok(())
    }

    fn begin_scope(&mut self) {
        todo!()
    }

    fn end_scope(&mut self) {
        todo!()
    }

    fn resolve_stmt(&mut self, statement: &Stmt) -> Result<()> {
        todo!()
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

    fn declare(&mut self, name: &Token) -> Result<()> {
        if !self.scopes.is_empty() {
            self.scopes[self.scopes.len() - 1].insert(name.lexeme.clone(), false)
        }
        Ok(())
    }

    fn define(&mut self, name: &Token) -> Result<()> {
        if !self.scopes.is_empty() {
            self.scopes[self.scopes.len() - 1].insert(name.lexeme.clone(), true)
        }
        Ok(())
    }
}
