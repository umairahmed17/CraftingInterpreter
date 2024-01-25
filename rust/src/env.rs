use std::collections::HashMap;

use crate::{
    error::Error,
    expr::{Symbol, Value},
};

#[derive(Debug)]
pub struct Environment {
    pub values: HashMap<Symbol, Value>,
}

impl Environment {
    pub fn define(&mut self, name: &Symbol, val: Value) {
        self.values.insert(name.clone(), val);
    }

    pub fn get(&self, name: &Symbol) -> Result<Value, Error> {
        if !self.values.contains_key(&name) {
            let err = format!("Undefined variable {0}.", name.name);
            return Err(Error::RunTimeException {
                message: err,
                line: name.line,
                col: name.col,
            });
        }
        let val = self.values.get(&name);
        match val {
            Some(val) => Ok(val.clone()),
            None => {
                let err = format!("Undefined variable {0}.", name.name);
                return Err(Error::RunTimeException {
                    message: err,
                    line: name.line,
                    col: name.col,
                });
            }
        }
    }
}
