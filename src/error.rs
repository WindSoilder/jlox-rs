use crate::{Token, TokenType};
pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn error_at_token(token: Token, message: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, "at end", message);
    } else {
        report(token.line, &format!("at '{}'", token.lexeme), message);
    }
}

fn report(line: usize, pos: &str, message: &str) {
    let err_text = format!("[line {line}] Error {pos}: {message}");
    eprintln!("{}", err_text);
}
