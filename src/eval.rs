use std::fmt::Display;

use crate::TokenType;
use crate::error::JloxError;
use crate::scanner::Literal;
use crate::{Environment, Expr, Stmt};
use anyhow::Result;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<()> {
        for one_stmt in statements {
            self.execute(one_stmt)?
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<()> {
        match statement {
            Stmt::Print(expr) => {
                let result = self.evaluate(expr)?;
                println!("{}", result);
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Var(var_decl) => {
                let mut value = Value::Null;
                if let Some(init_val) = &var_decl.initializer {
                    value = self.evaluate(init_val)?;
                }
                self.environment.define(var_decl.name.lexeme.clone(), value);
            }
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value> {
        let result = match expr {
            Expr::Literal(lit) => match lit {
                Literal::String(s) => Value::String(s.to_string()),
                Literal::Nil => Value::Null,
                Literal::Number(n) => Value::Number(*n),
                Literal::Bool(b) => Value::Bool(*b),
            },
            Expr::Grouping(g) => self.evaluate(g.as_ref())?,
            Expr::Unary((op, expr)) => {
                let right = self.evaluate(expr.as_ref())?;

                match op.token_type {
                    TokenType::Minus => {
                        if let Value::Number(n) = right {
                            Value::Number(-n)
                        } else {
                            return Err(eval_error(op.line, "Operand must be a number"));
                        }
                    }
                    TokenType::Bang => Value::Bool(!is_truthy(&right)),
                    _ => return Err(eval_error(op.line, "unary operator must by '-' or '!'")),
                }
            }
            Expr::Binary((left, op, right)) => {
                let left = self.evaluate(left.as_ref())?;
                let right = self.evaluate(right.as_ref())?;
                // separate out for `!=` and `==` operator
                match op.token_type {
                    TokenType::BangEqual => return Ok(Value::Bool(left != right)),
                    TokenType::EqualEqual => return Ok(Value::Bool(left == right)),
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
                        invalid => {
                            return Err(eval_error(
                                op.line,
                                format!("invalid operator {:?}", invalid),
                            ));
                        }
                    },
                    (Value::String(l), Value::String(r)) => match op.token_type {
                        TokenType::Plus => Value::String(format!("{l}{r}")),
                        _ => return Err(eval_error(op.line, "String only support '+' operator")),
                    },
                    _ => {
                        return Err(eval_error(
                            op.line,
                            "Operands must be two numbers or two strings",
                        ));
                    }
                };
                result
            }
            Expr::Var(token) => self.environment.get(token)?.clone(),
            Expr::Assignment((name, value)) => {
                let value = self.evaluate(value)?;
                self.environment.assign(name, value.clone())?;
                value
            },
            Expr::Garbage => return Err(eval_error(0, "Get garbage result")),
        };
        Ok(result)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    String(String),
    Null,
    Number(f64),
    Bool(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Null => write!(f, "nil"),
            Value::Number(n) => write!(f, "{n}"),
            Value::Bool(b) => write!(f, "{b}"),
        }
    }
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Null => false,
        Value::Bool(b) => *b,
        _ => true,
    }
}

fn eval_error(line: usize, message: impl Into<String>) -> anyhow::Error {
    JloxError::EvalError {
        line: line as u32,
        message: message.into(),
    }
    .into()
}
