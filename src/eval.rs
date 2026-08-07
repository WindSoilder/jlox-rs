use std::fmt::Display;

use crate::TokenType;
use crate::callable::{Callable, CallableId, Clock, CustomCallable};
use crate::error::JloxError;
use crate::scanner::Literal;
use crate::{Environment, Expr, Stmt};
use anyhow::Result;
use std::mem;
use std::sync::Arc;

pub struct Interpreter {
    pub callables: Vec<std::sync::Arc<dyn Callable>>,
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut global = Environment::new(None);
        let mut callables = vec![];
        define_callable(
            "clock".to_string(),
            Arc::new(Clock),
            &mut callables,
            &mut global,
        );
        Self {
            callables,
            environment: Environment::new(None),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<()> {
        for one_stmt in statements {
            self.execute(one_stmt)?
        }
        Ok(())
    }

    pub fn execute(&mut self, statement: &Stmt) -> Result<()> {
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
            Stmt::Block(block) => {
                let old_env = mem::take(&mut self.environment);
                self.environment = Environment::new(Some(Box::new(old_env)));
                for one_stmt in &block.statements {
                    self.execute(one_stmt)?;
                }
                self.environment = self.environment.into_enclosing()
            }
            Stmt::If(if_stmt) => {
                if is_truthy(&self.evaluate(&if_stmt.condition)?) {
                    self.execute(&if_stmt.then_branch)?;
                } else {
                    match &if_stmt.else_branch {
                        None => (),
                        Some(else_branch) => {
                            self.execute(else_branch)?;
                        }
                    }
                }
            }
            Stmt::While(while_stmt) => {
                while is_truthy(&self.evaluate(&while_stmt.condition)?) {
                    self.execute(&while_stmt.body)?;
                }
            }
            Stmt::Func(func_decl) => {
                let callables = &mut self.callables;
                let env = &mut self.environment;
                let new_func = CustomCallable {
                    decl: func_decl.clone(),
                };

                define_callable(
                    func_decl.name.lexeme.clone(),
                    Arc::new(new_func),
                    callables,
                    env,
                );
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
            }
            Expr::Logical((left, op, right)) => {
                let left = self.evaluate(left)?;

                if op.token_type == TokenType::Or {
                    if is_truthy(&left) {
                        return Ok(left);
                    }
                } else {
                    if !is_truthy(&left) {
                        return Ok(left);
                    }
                }
                self.evaluate(right)?
            }
            Expr::Call((callee, paren, arguments)) => {
                let callee = self.evaluate(callee)?;
                // make sure that it's callable.
                if matches!(callee, Value::Callable(_)) == false {
                    return Err(eval_error(
                        paren.line,
                        "Can only call functions and classes.",
                    ));
                }
                if arguments.len() != callee.arity(self) {
                    return Err(eval_error(
                        paren.line,
                        format!(
                            "Expected {} arguments but got {}.",
                            callee.arity(self),
                            arguments.len()
                        ),
                    ));
                }

                let mut args = vec![];
                for arg in arguments {
                    args.push(self.evaluate(arg)?);
                }

                callee.call(self, args)?
            }
            Expr::Garbage => return Err(eval_error(0, "Get garbage result")),
        };
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Null,
    Number(f64),
    Bool(bool),
    Callable(CallableId),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (String(a), String(b)) => a == b,
            (Null, Null) => true,
            (Number(a), Number(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl Value {
    fn call(self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
        if let Value::Callable(call_id) = self {
            let callable = interpreter.callables[call_id.0].clone();
            callable.call(interpreter, args)
        } else {
            panic!("should make sure that the value is callable")
        }
    }

    fn arity(&self, interpreter: &Interpreter) -> usize {
        if let Value::Callable(call_id) = self {
            let callable = &interpreter.callables[call_id.0];
            callable.arity()
        } else {
            panic!("should make sure that the value is callable")
        }
    }

    fn is_callable(&self) -> bool {
        matches!(self, Value::Callable(_))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Null => write!(f, "nil"),
            Value::Number(n) => write!(f, "{n}"),
            Value::Bool(b) => write!(f, "{b}"),
            // FIXME: oops, failed to get function name.
            Value::Callable(c) => write!(f, "callable func"),
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

fn define_callable(
    name: String,
    callable: Arc<dyn Callable>,
    result: &mut Vec<Arc<dyn Callable>>,
    env: &mut Environment,
) {
    result.push(callable);
    let callable_id = CallableId(result.len() - 1);
    env.define(name, Value::Callable(callable_id));
}
