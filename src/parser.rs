use crate::error::error_at_token;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::{Literal, Token, TokenType};

pub struct ParseError {
    token: Token,
    message: String,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    parse_errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            parse_errors: vec![],
        }
    }

    pub fn parse(&mut self) -> Option<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            match self.statement() {
                Some(stmt) => statements.push(stmt),
                None => return None,
            }
        }
        Some(statements)
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.is_match(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.")
            .map(|_| Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")
            .map(|_| Stmt::Expression(expr))
    }

    pub fn parse_back(&mut self) -> Option<Expr> {
        let prev_error_len = self.parse_errors.len();
        let expr = self.expression();
        let error_len = self.parse_errors.len();
        if error_len > prev_error_len {
            None
        } else {
            Some(expr)
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
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
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary((Box::new(expr), operator, Box::new(right)));
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary((Box::new(expr), operator, Box::new(right)));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary((Box::new(expr), operator, Box::new(right)));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            Expr::Unary((operator, Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.is_match(&[TokenType::False]) {
            return Expr::Literal(Literal::Bool(false));
        } else if self.is_match(&[TokenType::True]) {
            return Expr::Literal(Literal::Bool(true));
        } else if self.is_match(&[TokenType::Nil]) {
            return Expr::Literal(Literal::Nil);
        } else if self.is_match(&[TokenType::Number, TokenType::String]) {
            let literal = self
                .previous()
                .literal
                .clone()
                .expect("literal token should carry a value");
            return Expr::Literal(literal);
        } else if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression();
            if self
                .consume(TokenType::RightParen, "Expect ')' after expression.")
                .is_none()
            {
                return Expr::Garbage;
            } else {
                return Expr::Grouping(Box::new(expr));
            }
        }

        self.error(self.peek().clone(), "Expect expression.");
        Expr::Garbage
    }

    fn error(&mut self, token: Token, message: &str) {
        let parse_error = ParseError {
            token: token.clone(),
            message: message.to_string(),
        };
        self.parse_errors.push(parse_error);
        error_at_token(&token, message);
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Option<Token> {
        if self.check(token_type) {
            Some(self.advance())
        } else {
            self.error(self.peek().clone(), message);
            None
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}
