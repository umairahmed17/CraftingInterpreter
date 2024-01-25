use crate::{
    env::Environment,
    error::Error,
    expr::{Stmt, Value},
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
            Stmt::Print(v) => match v.get_value() {
                Ok(v) => {
                    println!("{v:?}");
                    return Ok(v);
                }
                Err(e) => return Err(e),
            },
            Stmt::VarDecl(sym, expr) => {
                let mut value = Value::Nil;
                match expr {
                    Some(expr) => {
                        value = expr.get_value()?;
                    }
                    None => {
                        return self.env.get(&sym);
                    },
                }
                if let Some(expr) = expr {
                    value = expr.get_value()?;
                }
                self.env.define(sym, value);
                return Ok(Value::Nil);
            }
            _ => return Ok(Value::Nil),
        }
    }
}
