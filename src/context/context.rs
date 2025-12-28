use std::collections::HashMap;
use crate::errors::error::ExpressionError;

#[derive(Debug)]
pub struct Context {
    vars: HashMap<String, i32>,
}

impl Context {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    pub fn set(&mut self, k: &str, v: i32) {
        self.vars.insert(k.to_string(), v);
    }

    pub fn get(&self, k: &str) -> Result<i32, ExpressionError> {
        self.vars.get(k).copied().ok_or(
            ExpressionError::Evaluation {
                message: format!("Undefined variable '{}'", k),
            }
        )
    }
}
