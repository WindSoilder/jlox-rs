use crate::error;
use std::{any::Any, fmt::Display};

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.into_bytes(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), self.line));
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        use TokenType::*;
        let ch = self.advance();
        match ch {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let token = if self.peek_match(b'=') {
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(token)
            }
            '=' => {
                let token = if self.peek_match(b'=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(token)
            }
            '<' => {
                let token = if self.peek_match(b'=') {
                    LessEqual
                } else {
                    Less
                };
                self.add_token(token)
            }
            '>' => {
                let token = if self.peek_match(b'=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(token)
            }
            '/' => {
                if self.peek_match(b'/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => (), // Ignore whitespace.
            '\n' => {self.line += 1;}
            '"' => self.string(),
            _ => error(self.line, "Unexpected character."),
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes.
        let value = String::from_utf8_lossy(&self.source[self.start + 1..self.current - 1]);
        self.add_token(TokenType::String, value);

    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current] as char
        }
    }

    fn peek_match(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            false
        } else if self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> char {
        let result = self.source[self.current] as char;
        self.current += 1;
        result
    }

    fn add_token(&mut self, token: TokenType) {
        let one_token = Token::new(
            token,
            String::from_utf8_lossy(&self.source[self.current..self.current]).into_owned(),
            Box::new(Option::<()>::None),
            self.line,
        );
        self.tokens.push(one_token)
    }
    
    fn add_token_with_literal(&mut self, token: TokenType, literal: Box<dyn Any>) {

    }
}

// TODO: change lexeme from String to &[u8]
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Box<dyn Any>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Box<dyn Any>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexeme)
    }
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, Ture, Var, While,

    Eof
}
