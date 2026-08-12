// TODO: remove it, allow it for now to reduce IDE noise.
#![allow(unused)]

use crate::{Block, Expr, Stmt, Token};
use anyhow::Result;
use std::collections::HashMap;

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

    fn declare(&mut self, token: &Token) -> Result<()> {
        todo!()
    }

    fn define(&mut self, token: &Token) -> Result<()> {
        todo!()
    }
    fn resolve_expr(&mut self, expr: &Expr) -> Result<()> {
        todo!()
    }
}
