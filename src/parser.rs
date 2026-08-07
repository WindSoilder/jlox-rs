use crate::error::error_at_token;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::{Block, FuncDecl, If, Literal, Token, TokenType, VarDecl, While};

pub struct ParseError {
    token: Token,
    message: String,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub parse_errors: Vec<ParseError>,
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
            match self.declaration() {
                Some(stmt) => statements.push(stmt),
                None => return None,
            }
        }
        Some(statements)
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let result = if self.is_match(&[TokenType::Var]) {
            self.var_declaration()
        } else if self.is_match(&[TokenType::Def]) {
            self.function("function")
        } else {
            self.statement()
        };
        if result.is_none() {
            self.synchronize();
        }
        result
    }

    fn function(&mut self, kind: &str) -> Option<Stmt> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {kind} name"))?;
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {kind} name"),
        )?;
        let mut parameters = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    error_at_token(self.peek(), "Can't have more than 255 parameters.");
                }

                parameters.push(self.consume(TokenType::Identifier, "Epxect parameter name.")?);

                if !self.is_match(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after paremters.")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {kind} body."),
        )?;
        let body = self.block()?;
        Some(Stmt::Func(FuncDecl::new(name, parameters, body)))
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect variable name")?;
        let mut initializer = None;
        if self.is_match(&[TokenType::Equal]) {
            initializer = Some(self.expression());
            if matches!(initializer, Some(Expr::Garbage)) {
                return None;
            }
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );
        Some(Stmt::Var(VarDecl::new(name, initializer)))
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.is_match(&[TokenType::Print]) {
            self.print_statement()
        } else if self.is_match(&[TokenType::LeftBrace]) {
            Some(Stmt::Block(Block::new(self.block()?)))
        } else if self.is_match(&[TokenType::If]) {
            self.if_statement()
        } else if self.is_match(&[TokenType::While]) {
            self.while_statement()
        } else if self.is_match(&[TokenType::For]) {
            self.for_statement()
        } else {
            self.expression_statement()
        }
    }

    fn for_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.is_match(&[TokenType::Semicolon]) {
            None
        } else if self.is_match(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.expression_statement()
        };

        let mut condition = if self.check(TokenType::Semicolon) {
            None
        } else {
            let repr = self.expression();
            if repr.is_garbage() {
                return None;
            }
            Some(repr)
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut incr = if self.check(TokenType::RightParen) {
            None
        } else {
            let repr = self.expression();
            if repr.is_garbage() {
                return None;
            }
            Some(repr)
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;
        if incr.is_some() {
            let incr = incr.expect("already check exists");
            body = Stmt::Block(Block::new(vec![body, Stmt::Expression(incr)]));
        }

        if condition.is_none() {
            condition = Some(Expr::Literal(Literal::Bool(true)));
        }
        body = Stmt::While(While::new(
            condition.expect("the incremental should always contains something"),
            Box::new(body),
        ));

        if initializer.is_some() {
            let initializer = initializer.expect("already check exists");
            body = Stmt::Block(Block::new(vec![initializer, body]));
        }
        Some(body)
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'");
        let condition = self.expression();
        if condition.is_garbage() {
            return None;
        }
        self.consume(TokenType::RightParen, "Expect ')' after condition");
        let body = self.statement()?;
        Some(Stmt::While(While::new(condition, Box::new(body))))
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression();
        if condition.is_garbage() {
            return None;
        }
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        let then_branch = self.statement()?;

        let else_branch = if self.is_match(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Some(Stmt::If(If::new(
            condition,
            Box::new(then_branch),
            else_branch,
        )))
    }

    fn block(&mut self) -> Option<Vec<Stmt>> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Some(statements)
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
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.or();

        if self.is_match(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment();

            if let Expr::Var(token) = expr {
                return Expr::Assignment((token, Box::new(value)));
            }
            self.error(equals, "Invalid assignment target.");
            return Expr::Garbage;
        }
        expr
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();

        while self.is_match(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and();
            expr = Expr::Logical((Box::new(expr), operator, Box::new(right)));
        }
        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();

        while self.is_match(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality();
            expr = Expr::Logical((Box::new(expr), operator, Box::new(right)));
        }
        expr
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
            self.call()
        }
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();

        loop {
            if self.is_match(&[TokenType::LeftParen]) {
                let parsed_call = self.finish_call(expr);
                match parsed_call {
                    None => return Expr::Garbage,
                    Some(parsed_call) => expr = parsed_call,
                }
            } else {
                break;
            }
        }
        expr
    }

    fn finish_call(&mut self, callee: Expr) -> Option<Expr> {
        let mut arguments = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                let expr = self.expression();
                if expr.is_garbage() {
                    return None;
                }
                if arguments.len() >= 255 {
                    error_at_token(self.peek(), "Can't have more than 255 arguments.");
                }
                arguments.push(expr);

                if !self.is_match(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Some(Expr::Call((Box::new(callee), paren, arguments)))
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
        } else if self.is_match(&[TokenType::Identifier]) {
            return Expr::Var(self.previous().clone());
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
                | TokenType::Def
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
