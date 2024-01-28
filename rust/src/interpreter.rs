use crate::{
    env::Environment,
    error::Error,
    expr::{BinaryOp, BinaryOpTy, Expr, Literal, Stmt, UnaryOp, UnaryOpTy, Value},
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
            Stmt::Expr(v) => match self.get_value(v) {
                Ok(v) => {
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
                let value = self.get_value(v)?;
                println!("The value is: {0}", value);
                return Ok(value.clone());
            }
            Stmt::VarDecl(sym, expr) => {
                let mut value = Value::Nil;
                match expr {
                    Some(expr) => {
                        value = self.get_value(expr)?;
                    }
                    None => {
                        return self.env.get(&sym);
                    }
                }
                if let Some(expr) = expr {
                    value = self.get_value(expr)?;
                }
                self.env.define(sym, value.clone());
                return Ok(value);
            }
            Stmt::Assign(symbol, stmt) => {
                println!("The symbol: {symbol:?}");
                let value = self.evaluate(stmt)?;
                println!("The value: {value:?}");
                let _ = self.env.assign(symbol, value.clone())?;
                return Ok(value);
            }
            _ => return Ok(Value::Nil),
        }
    }

    pub fn get_value(&mut self, expr: &Expr) -> Result<Value, Error> {
        match expr {
            Expr::Literal(val) => match val {
                Literal::Number(val) => Ok(Value::Number(*val)),
                Literal::String(val) => Ok(Value::String(val.to_string())),
                Literal::True => Ok(Value::Bool(true)),
                Literal::False => Ok(Value::Bool(true)),
                Literal::Nil => Ok(Value::Nil),
            },
            Expr::Unary(op, expr) => self.interpret_unary(op, expr),
            Expr::Binary(left_expr, op, right_expr) => {
                self.interpret_binary(left_expr, right_expr, op)
            }
            Expr::Grouping(expr) => return self.get_value(expr),
            Expr::Assign(symbol, expr) => {
                let value = self.get_value(expr)?;
                let _ = self.env.assign(symbol, value.clone());
                return Ok(value);
            }
            Expr::Variable(v) => {
                let msg = format!("Variable: `{0}` is undefined.", v.name);

                if self.env.values.contains_key(&v.name) {
                    return Ok(self.env.values.get(&v.name).unwrap().clone());
                }
                return Err(Error::RunTimeException {
                    message: msg,
                    line: v.line,
                    col: v.col,
                });
            }
            _ => return Ok(Value::Nil),
        }
    }

    fn interpret_unary(&mut self, op: &UnaryOp, right: &Expr) -> Result<Value, Error> {
        let right: Value = self.get_value(right)?;
        match op.ty {
            UnaryOpTy::Minus => {
                if let Value::Number(v) = right {
                    return Ok(Value::Number(-v));
                }
                let message = "Wrong Unary Token In {op:?} with {expr:?}";
                return Err(Error::RunTimeException {
                    message: String::from(message),
                    line: op.line,
                    col: op.col,
                });
            }
            UnaryOpTy::Bang => {
                if let Value::Number(v) = right {
                    return Ok(Value::Bool(is_truthy(v)));
                }
                let message = "Wrong Unary Token In {op:?} with {expr:?}";
                return Err(Error::RunTimeException {
                    message: String::from(message),
                    line: op.line,
                    col: op.col,
                });
            }
        }
    }

    fn interpret_binary(
        &mut self,
        left_expr: &Expr,
        right_expr: &Expr,
        op: &BinaryOp,
    ) -> Result<Value, Error> {
        let right = self.get_value(right_expr)?;
        let left = self.get_value(left_expr)?;

        if let Value::Number(l) = left {
            if let Value::Number(r) = right {
                match op.ty {
                    BinaryOpTy::EqualEqual => return Ok(Value::Bool(l == r)),
                    BinaryOpTy::NotEqual => return Ok(Value::Bool(l != r)),
                    BinaryOpTy::Less => return Ok(Value::Bool(l < r)),
                    BinaryOpTy::LessEqual => return Ok(Value::Bool(l <= r)),
                    BinaryOpTy::Greater => return Ok(Value::Bool(l > r)),
                    BinaryOpTy::GreaterEqual => return Ok(Value::Bool(l >= r)),
                    BinaryOpTy::Plus => return Ok(Value::Number(l + r)),
                    BinaryOpTy::Minus => return Ok(Value::Number(l - r)),
                    BinaryOpTy::Star => return Ok(Value::Number(l * r)),
                    BinaryOpTy::Slash => return Ok(Value::Number(l / r)),
                }
            }
        }
        if let Value::String(ref l) = left {
            if let Value::String(r) = right {
                match op.ty {
                    BinaryOpTy::Plus => {
                        let mut s = l.to_owned();
                        s.push_str(&r);
                        return Ok(Value::String(s));
                    }
                    _ => {
                        let message =
                            "Wrong Binary Token In {op:?} with {right_expr:?} and {left_expr:?}";
                        return Err(Error::RunTimeException {
                            message: String::from(message),
                            line: op.line,
                            col: op.col,
                        });
                    }
                }
            }
        }
        if let Value::Bool(l) = left {
            if let Value::Bool(r) = right {
                match op.ty {
                    BinaryOpTy::EqualEqual => return Ok(Value::Bool(l == r)),
                    BinaryOpTy::NotEqual => return Ok(Value::Bool(l != r)),
                    _ => {
                        let message =
                            "Wrong Binary Token In {op:?} with {right_expr:?} and {left_expr:?}";
                        return Err(Error::RunTimeException {
                            message: String::from(message),
                            line: op.line,
                            col: op.col,
                        });
                    }
                }
            }
        }
        if let Value::Nil = left {
            if let Value::Nil = right {
                match op.ty {
                    BinaryOpTy::EqualEqual => return Ok(Value::Bool(true)),
                    BinaryOpTy::NotEqual => return Ok(Value::Bool(false)),
                    _ => {
                        let message =
                            "Wrong Binary Token In {op:?} with {right_expr:?} and {left_expr:?}";
                        return Err(Error::RunTimeException {
                            message: String::from(message),
                            line: op.line,
                            col: op.col,
                        });
                    }
                }
            }
        }
        let message = "Wrong Binary Token In {op:?} with {right_expr:?} and {left_expr:?}";
        return Err(Error::RunTimeException {
            message: String::from(message),
            line: op.line,
            col: op.col,
        });
    }
}

fn is_truthy(v: f64) -> bool {
    if v > 0.0 {
        return true;
    }
    return false;
}
