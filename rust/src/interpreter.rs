use std::collections::HashMap;
use std::time::SystemTime;

use crate::{
    env::Environment,
    error::Error,
    expr::{BinaryOp, BinaryOpTy, Expr, Literal, LogicalOp, Stmt, UnaryOp, UnaryOpTy, Value},
};

pub struct Interpreter<'a> {
    pub statements: &'a Vec<Stmt>,
    pub env: Environment,
    loop_stack: Vec<String>,
}

impl<'a> Interpreter<'a> {
    pub fn from_statements(statements: &'a Vec<Stmt>) -> Self {
        let environment = Environment {
            values: HashMap::new(),
            enclosing: None,
        };
        return Self {
            statements,
            env: environment,
            loop_stack: vec![],
        };
    }
    pub fn interpret(&mut self) -> Result<(), Error> {
        for stmt in self.statements {
            if let Err(res) = self.evaluate(stmt) {
                return Err(res);
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::Expr(v) => match self.get_value(v) {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => return Err(e),
            },
            Stmt::Print(v) => {
                let value = self.get_value(v)?;
                println!("The value is: {0}", value);
                return Ok(());
            }
            Stmt::VarDecl(sym, expr) => match expr {
                Some(expr) => {
                    let value = self.get_value(expr)?;
                    self.env.define(sym, value.clone());
                    return Ok(());
                }
                None => {
                    self.env.define(sym, Value::Nil);
                    return Ok(());
                }
            },
            Stmt::Block(statements) => {
                self.env = Environment::with_enclosing(self.env.clone());

                for statement in statements {
                    if let Stmt::Break = statement {
                        self.loop_stack.pop();
                        break;
                    }
                    let _ = self.evaluate(statement)?;
                }
                if let Some(enclosing) = self.env.enclosing.clone() {
                    self.env = *enclosing;
                }
                return Ok(());
            }
            Stmt::While(condition, while_stmt) => {
                let id = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs()
                    .to_string();
                self.loop_stack.push(id.clone());
                while is_truthy(&self.get_value(condition)?)?
                    && self.loop_stack.last().unwrap_or(&"None".to_string()) == &id
                {
                    self.evaluate(while_stmt)?;
                }
                return Ok(());
            }
            Stmt::If(condition, if_stmt, else_stmt) => {
                if is_truthy(&self.get_value(condition)?)? {
                    self.evaluate(if_stmt)?;
                } else if let Some(else_stmt) = else_stmt {
                    self.evaluate(else_stmt)?;
                }
                return Ok(());
            }
            _ => return Ok(()),
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
                return self.env.get(&v);
            }
            Expr::Logical(left, op, right) => {
                return self.interpret_logical(left, op, right);
            }
        }
    }

    fn interpret_logical(
        &mut self,
        left: &Box<Expr>,
        op: &LogicalOp,
        right: &Box<Expr>,
    ) -> Result<Value, Error> {
        let left = self.get_value(left)?;
        if let LogicalOp::Or = op {
            if is_truthy(&left)? {
                return Ok(left);
            }
        }
        if let LogicalOp::And = op {
            if !is_truthy(&left)? {
                return Ok(left);
            }
        }
        return self.get_value(right);
    }

    fn interpret_unary(&mut self, op: &UnaryOp, right: &Expr) -> Result<Value, Error> {
        let right: Value = self.get_value(right)?;
        match op.ty {
            UnaryOpTy::Minus => {
                if let Value::Number(v) = right {
                    return Ok(Value::Number(-v));
                }
                let message = format!("Wrong Unary Token In {op:?} with {right:?}");
                return Err(Error::RunTimeException {
                    message: String::from(message),
                    line: op.line,
                    col: op.col,
                });
            }
            UnaryOpTy::Bang => {
                return Ok(Value::Bool(is_truthy(&right)?));
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
            if let Value::String(ref r) = right {
                match op.ty {
                    BinaryOpTy::Plus => {
                        let mut s = l.to_owned();
                        s.push_str(&r);
                        return Ok(Value::String(s));
                    }
                    BinaryOpTy::EqualEqual => return Ok(Value::Bool(l == r)),
                    BinaryOpTy::NotEqual => return Ok(Value::Bool(l != r)),
                    _ => {
                        let message = format!(
                            "Wrong Binary Token In {op:?} with {right_expr:?} and {left_expr:?}"
                        );
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
                        let message = format!(
                            "Wrong Binary Token In {op:?} with {right_expr:?} and {left_expr:?}"
                        );
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
                        let message = format!(
                            "Wrong Binary Token In {op:?} with {right_expr:?} and {left_expr:?}"
                        );
                        return Err(Error::RunTimeException {
                            message: String::from(message),
                            line: op.line,
                            col: op.col,
                        });
                    }
                }
            }
        }
        let message = format!("Wrong Binary Token In {op:?} with {right_expr:?} and {left_expr:?}");
        return Err(Error::RunTimeException {
            message: String::from(message),
            line: op.line,
            col: op.col,
        });
    }
}

fn is_truthy(val: &Value) -> Result<bool, Error> {
    match val {
        Value::String(_) => {
            return Err(Error::JustError {
                message: String::from("Cannot cast string to bool"),
            });
        }
        Value::Number(v) => {
            if *v > 0.0 {
                return Ok(true);
            }
            return Ok(false);
        }
        Value::Bool(v) => {
            return Ok(*v);
        }
        Value::Nil => {
            return Err(Error::JustError {
                message: String::from("Nil cannot be casted to bool"),
            })
        }
    }
}
