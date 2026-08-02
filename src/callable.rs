use crate::{Interpreter, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Debug;

pub trait Callable: Debug + Clone {
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
