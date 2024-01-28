use std::{collections::HashMap, fmt::Display};

use crate::{
    error::Error,
    expr::{Symbol, Value},
};

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Value>,
    pub enclosing: Option<Box<Environment>>,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut enclosing = None;
        if let Some(ref enc) = self.enclosing {
            enclosing = Some(&enc.values);
        }
        return write!(
            f,
            "Environment Values: {0:?}, enclosing: `{enclosing:?}`",
            self.values
        );
    }
}

impl Environment {
    pub fn define(&mut self, name: &Symbol, val: Value) {
        self.values.insert(name.name.clone(), val);
    }

    pub fn get(&self, name: &Symbol) -> Result<Value, Error> {
        if !self.values.contains_key(&name.name) {
            if let Some(ref enclosing) = self.enclosing {
                return enclosing.get(name);
            }
            let err = format!("Undefined variable {0}.", name.name);
            return Err(Error::RunTimeException {
                message: err,
                line: name.line,
                col: name.col,
            });
        }
        let val = self.values.get(&name.name);
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

    pub fn assign(&mut self, symbol: &Symbol, value: Value) -> Result<(), Error> {
        if let Some(ref mut enclosing) = self.enclosing {
            return enclosing.assign(symbol, value);
        }
        if self.values.contains_key(&symbol.name) {
            *self.values.get_mut(&symbol.name).unwrap() = value;
            return Ok(());
        }
        let msg = format!("Undefined Variable `{0}`.", symbol.name);
        return Err(Error::RunTimeException {
            message: msg,
            line: symbol.line,
            col: symbol.col,
        });
    }
}
