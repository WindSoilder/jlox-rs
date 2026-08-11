use crate::callable::{self, Callable, CallableId};
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;
use std::cell::Ref;
use std::cell::RefCell;

use crate::{JloxError, Token, Value};

#[derive(Default, Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    // FIXME: can I implement it without Clone?
    pub fn get(&self, name: &Token) -> Result<Value, JloxError> {
        match self.values.get(&name.lexeme) {
            Some(val) => return Ok(val.clone()),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    let x = enclosing.borrow();
                    return x.get(name).map(|v| v.clone());
                }
            }
        }
        Err(JloxError::EvalError {
            line: name.line as u32,
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }

    pub fn into_enclosing(&mut self) -> Rc<RefCell<Environment>> {
        let result = mem::take(&mut self.enclosing).unwrap();
        result
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), JloxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        } else {
            if let Some(enclosing) = &self.enclosing {
                enclosing.borrow_mut().assign(name, value)?;
                return Ok(());
            }
        }
        Err(JloxError::EvalError {
            line: name.line as u32,
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }
}
