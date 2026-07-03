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

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary((Box::new(expr), operator, Box::new(right)));
        }
        expr
    }

    fn is_match(&mut self, token_types: &[TokenType]) -> bool {
        for t in token_types {
            if self.check(*t) {
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

    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary((Box::new(expr), operator, Box::new(right)));
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary((Box::new(expr), operator, Box::new(right)));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary((Box::new(expr), operator, Box::new(right)));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            Expr::Unary((operator, Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.is_match(&[TokenType::False]) {
            Expr::Literal(Box::new(false))
        } else if self.is_match(&[TokenType::True]) {
            Expr::Literal(Box::new(true))
        } else if self.is_match(&[TokenType::Nil]) {
            Expr::Literal(Box::new(None))
        } else if self.is_match(&[TokenType::Number, TokenType::String]) {
            Expr::Literal(self.previous().literal.clone())
        } else if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            Expr::Grouping(Box::new(expr))
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token{
        if self.check(token_type) {
            self.advance()
        } else {
            self.error(self.peek(), message)
        }
    }
}
