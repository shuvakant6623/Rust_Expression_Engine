use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
    variables: HashMap<String, i32>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: i32) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> i32 {
        *self.variables
            .get(name)
            .expect("Undefined variable")
    }
}