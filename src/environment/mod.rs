use std::collections::HashMap;

use crate::interpreter::{LoxObject, RuntimeError};

struct Environment {
    variables: HashMap<String, LoxObject>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    fn define(&mut self, name: String, value: LoxObject) {
        self.variables.insert(name, value);
    }

    fn get(&self, name: &str) -> Result<&LoxObject, RuntimeError> {
        self.variables
            .get(name)
            .ok_or_else(|| RuntimeError::UndefinedVariable(name.to_string()))
    }
}
