struct Resolver;
use crate::{Block, Expr, Stmt};
use anyhow::Result;

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
        todo!()
    }
}
