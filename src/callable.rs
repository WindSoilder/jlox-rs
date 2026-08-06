use crate::{Environment, FuncDecl, Interpreter, Value};
use anyhow::Result;
use std::fmt::Debug;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub struct CallableId(pub usize);

pub trait Callable: Debug {
    fn arity(&self) -> usize {
        self.get_decl().map(|decl| decl.params.len()).unwrap_or(0)
    }

    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
        let decl = self.get_decl();
        if decl.is_none() {
            panic!(
                "Internal error!  The function doesn't have declaration, it happened when builtin function doesn't have a block, and it doesn't override call method"
            );
        }
        let decl = decl.unwrap();
        let old_env = mem::take(&mut interpreter.environment);
        interpreter.environment = Environment::new(Some(Box::new(old_env)));
        for (one_param, one_arg) in decl.params.iter().zip(arguments) {
            interpreter
                .environment
                .define(one_param.lexeme.clone(), one_arg)
        }
        for one_stmt in &decl.body {
            interpreter.execute(one_stmt)?;
        }
        interpreter.environment = interpreter.environment.into_enclosing();
        Ok(Value::Null)
    }

    fn to_string(&self) -> String {
        let decl = self.get_decl();
        decl.map_or_else(|| "<native fn>".to_string(), |d| format!("<fn {} >", d.name.lexeme))
    }

    fn get_decl(&self) -> Option<FuncDecl> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Clock;

impl Callable for Clock {
    fn call(&mut self, _interpreter: &mut Interpreter, _arguments: Vec<Value>) -> Result<Value> {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Ok(Value::Number(t))
    }
}
