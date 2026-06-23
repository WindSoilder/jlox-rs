use std::fmt::Display;
use std::sync::LazyLock;
use std::collections::HashMap;

static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| {
    use TokenType::*;
    let mut keywords = HashMap::new();
    keywords.insert("and".to_string(),And);
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
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
        }
    }

    pub fn scan_tokens(self) -> Vec<TokenType> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            start = current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "", line))
        self.tokens
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance()
        }

        // Look for fractional part
        if (self.peek() == '.' && self.is_digit(self.peek_next())) {
            // Consume the "."
            self.advance()

            while self.is_digit(self.peek()) {
                self.advance()
            }
        }

        // self.add_token(TokenType::Number, ..)
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
            self.advance()
        }
        // self.add_token(TokenType::Identifier)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }


}

struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
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
    Print, Return, Super, This, True, Var, While,

    Eof
}
