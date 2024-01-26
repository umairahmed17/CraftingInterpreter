use crate::{
    env::Environment,
    error::Error,
    expr::{Expr, Stmt, Value},
};

pub struct Interpreter<'a> {
    pub statements: &'a Vec<Stmt>,
    pub env: Environment,
}

impl<'a> Interpreter<'a> {
    pub fn interpret(&mut self) -> Result<(), Error> {
        for stmt in self.statements {
            if let Err(res) = self.evaluate(stmt) {
                return Err(res);
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, stmt: &Stmt) -> Result<Value, Error> {
        match stmt {
            Stmt::Expr(v) => match v.get_value() {
                Ok(v) => {
                    println!("{v:?}");
                    return Ok(v);
                }
                Err(e) => return Err(e),
            },
            Stmt::Print(v) => {
                if let Expr::Variable(sym) = v {
                    if self.env.values.contains_key(&sym.name) {
                        let val = self.env.values.get(&sym.name).unwrap();
                        println!("The value of variable `{0}` is: {1}", sym.name, val);
                        return Ok(val.clone());
                    }
                    let msg = format!("The variable `{0}` is undefined.", sym.name);
                    return Err(Error::RunTimeException {
                        message: msg,
                        line: sym.line,
                        col: sym.col,
                    });
                }
                return Err(Error::JustError);
            }
            Stmt::VarDecl(sym, expr) => {
                let mut value = Value::Nil;
                match expr {
                    Some(expr) => {
                        value = expr.get_value()?;
                    }
                    None => {
                        return self.env.get(&sym);
                    }
                }
                if let Some(expr) = expr {
                    value = expr.get_value()?;
                }
                self.env.define(sym, value.clone());
                return Ok(value);
            }
            Stmt::Assign(symbol, stmt) => {
                let value = self.evaluate(stmt)?;
                let _ = self.env.assign(symbol, value.clone())?;
                return Ok(value);
            }
            _ => return Ok(Value::Nil),
        }
    }
}
