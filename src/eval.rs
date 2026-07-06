use crate::TokenType;
use crate::parser::Expr;
use crate::scanner::Literal;

#[derive(Eq, PartialEq)]
enum Value {
    String(String),
    Null,
    Number(f64),
    Bool(bool),
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Null => false,
        Value::Bool(b) => *b,
        _ => true,
    }
}
pub fn evaluate(expr: &Expr) -> Value {
    match expr {
        Expr::Literal(lit) => match lit {
            Literal::String(s) => Value::String(s.to_string()),
            Literal::Nil => Value::Null,
            Literal::Number(n) => Value::Number(*n),
            Literal::Bool(b) => Value::Bool(*b),
        },
        Expr::Grouping(g) => evaluate(g.as_ref()),
        Expr::Unary((op, expr)) => {
            let right = evaluate(expr.as_ref());

            match op.token_type {
                TokenType::Minus if let Value::Number(n) = right => Value::Number(-n),
                TokenType::Bang => Value::Bool(!is_truthy(&right)),
                // NOTE: how to handle error?
                _ => unreachable!("."),
            }
        }
        Expr::Binary((left, op, right)) => {
            let left = evaluate(left.as_ref());
            let right = evaluate(right.as_ref());
            // separate out for `!=` and `==` operator
            match op.token_type {
                TokenType::BangEqual => return Value::Bool(left != right),
                TokenType::EqualEqual => return Value::Bool(left == right),
                _ => {}
            }

            let result = match (left, right) {
                (Value::Number(l), Value::Number(r)) => match op.token_type {
                    TokenType::Minus => Value::Number(l - r),
                    TokenType::Slash => Value::Number(l / r),
                    TokenType::Star => Value::Number(l * r),
                    TokenType::Plus => Value::Number(l + r),
                    TokenType::Greater => Value::Bool(l > r),
                    TokenType::GreaterEqual => Value::Bool(l >= r),
                    TokenType::Less => Value::Bool(l < r),
                    TokenType::LessEqual => Value::Bool(l <= r),
                    _ => unreachable!("."),
                },
                (Value::String(l), Value::String(r)) => match op.token_type {
                    TokenType::Plus => Value::String(format!("{l}{r}")),
                    _ => unreachable!("."),
                },
                _ => unreachable!("."),
            };
            result
        }
        _ => todo!(".."),
    }
}
