use std::collections::HashMap;

use crate::{JloxError, Token, Value};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<&Value, JloxError> {
        match self.values.get(&name.lexeme) {
            Some(val) => Ok(val),
            None => Err(JloxError::EvalError {
                line: name.line as u32,
                message: format!("Undefined variable '{}'.", name.lexeme),
            }),
        }
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), JloxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            Err(JloxError::EvalError {
                line: name.line as u32,
                message: format!("Undefined variable '{}'.", name.lexeme),
            })
        }
    }
}
