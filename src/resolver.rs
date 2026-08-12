use crate::{Block, Expr, Stmt};
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
        todo!()
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
        todo!()
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<()> {
        todo!()
    }
}
