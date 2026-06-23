use crate::error;
use std::any::Any;
use std::collections::HashMap;
use std::f64;
use std::fmt::Display;
use std::sync::LazyLock;

static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| {
    use TokenType::*;
    let mut keywords = HashMap::new();
    keywords.insert("and".to_string(), And);
    keywords.insert("class".to_string(), Class);
    keywords.insert("else".to_string(), Else);
    keywords.insert("false".to_string(), False);
    keywords.insert("for".to_string(), For);
    keywords.insert("fun".to_string(), Fun);
    keywords.insert("if".to_string(), If);
    keywords.insert("nil".to_string(), Nil);
    keywords.insert("or".to_string(), Or);
    keywords.insert("print".to_string(), Print);
    keywords.insert("return".to_string(), Return);
    keywords.insert("super".to_string(), Super);
    keywords.insert("this".to_string(), This);
    keywords.insert("true".to_string(), True);
    keywords.insert("var".to_string(), Var);
    keywords.insert("while".to_string(), While);
    keywords
});

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

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Box::new(Option::<()>::None),
            self.line,
        ));
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
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            _ => {
                if self.is_digit(ch) {
                    self.number()
                } else if self.is_alpha(ch) {
                    self.identifier()
                } else {
                    error(self.line, "Unexpected character.")
                }
            }
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
            return;
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes.
        let value =
            String::from_utf8_lossy(&self.source[self.start + 1..self.current - 1]).into_owned();
        self.add_token_with_literal(TokenType::String, Box::new(value));
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
            String::from_utf8_lossy(&self.source[self.start..self.current]).into_owned(),
            Box::new(Option::<()>::None),
            self.line,
        );
        self.tokens.push(one_token)
    }

    fn add_token_with_literal(&mut self, token: TokenType, literal: Box<dyn Any>) {
        let one_token = Token::new(
            token,
            String::from_utf8_lossy(&self.source[self.start..self.current]).into_owned(),
            literal,
            self.line,
        );
        self.tokens.push(one_token)
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let number = String::from_utf8_lossy(&self.source[self.start..self.current])
            .parse::<f64>()
            .expect("already checked to be a float");
        self.add_token_with_literal(TokenType::Number, Box::new(number))
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1] as char
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = String::from_utf8_lossy(&self.source[self.start..self.current]);
        let token_type = *KEYWORDS
            .get(text.as_ref())
            .unwrap_or(&TokenType::Identifier);
        self.add_token(token_type)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
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
#[derive(Debug, Clone, Copy)]
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
    Print, Return, Super, This, True, Var, While,

    Eof
}
