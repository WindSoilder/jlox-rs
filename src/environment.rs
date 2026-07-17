use std::collections::HashMap;

use crate::{JloxError, Token, Value};

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<&Value, JloxError> {
        match self.values.get(&name.lexeme) {
            Some(val) => return Ok(val),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.get(name);
                }
            }
        }
        Err(JloxError::EvalError {
            line: name.line as u32,
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), JloxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(())
        } else {
            if let Some(enclosing) = &mut self.enclosing {
                enclosing.assign(name, value)?;
                return Ok(())
            }
        }
        Err(JloxError::EvalError {
            line: name.line as u32,
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }
}
