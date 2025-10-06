use std::collections::HashMap;

use crate::interpreter::{LoxObject, RuntimeError};

pub struct Environment {
    variables: HashMap<String, LoxObject>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: LoxObject) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Result<&LoxObject, RuntimeError<'_>> {
        self.variables
            .get(name)
            .ok_or_else(|| RuntimeError::UndefinedVariable(name.to_string()))
    }
}
