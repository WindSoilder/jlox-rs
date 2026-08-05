use crate::{Interpreter, Value};
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub struct CallableId(pub usize);

pub trait Callable: Debug {
    fn arity(&self) -> usize {
        return 0;
    }
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value;
    fn to_string(&self) -> &'static str {
        "<native fn>"
    }
}

#[derive(Debug, Clone)]
pub struct Clock;

impl Callable for Clock {
    fn call(&mut self, _interpreter: &mut Interpreter, _arguments: Vec<Value>) -> Value {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Value::Number(t)
    }
}
