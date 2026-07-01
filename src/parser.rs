use crate::scanner::Token;
use crate::{Token, TokenType};
use std::any::Any;
enum Expr {
    Binary((Box<Expr>, Token, Box<Expr>)),
    Grouping(Box<Expr>),
    Literal(Box<dyn Any>),
    Unary((Token, Box<Expr>)),
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equiality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.is_match(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary((Box::new(expr), operator, Box::new(right)));
        }
        expr
    }

    fn is_match(&self, token_types: &[TokenType]) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }
}
