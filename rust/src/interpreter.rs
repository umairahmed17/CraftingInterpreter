use std::time::SystemTime;
use std::{collections::HashMap, time::UNIX_EPOCH};

use crate::expr::LoxFunction;
use crate::{
    env::Environment,
    error::Error,
    expr::{
        BinaryOp, BinaryOpTy, Expr, Literal, LogicalOp, NativeFunction, Stmt, Symbol, UnaryOp,
        UnaryOpTy, Value,
    },
};

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value, String>;
    fn arity(&self, interpreter: &Interpreter) -> u8;
}

pub struct Interpreter {
    pub globals: Environment,
    pub env: Environment,
    pub ret_val: Option<Value>,
    loop_stack: Vec<String>,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut globals = Environment {
            values: HashMap::new(),
            enclosing: None,
        };
        let clock_sym = Symbol {
            name: "clock".to_string(),
            line: 0,
            col: -1,
        };
        let clock = NativeFunction {
            name: clock_sym.name.clone(),
            arity: 0,
            callable: |_, _| {
                return Ok(Value::Number(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards.")
                        .as_secs() as f64,
                ));
            },
        };
        globals.define(&clock_sym, Value::NativeFunction(clock));
        return Self {
            globals: globals.clone(),
            env: globals,
            loop_stack: vec![],
            ret_val: None,
        };
    }
}

impl Interpreter {
    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<(), Error> {
        for stmt in stmts {
            if let Err(res) = self.evaluate(stmt) {
                return Err(res);
            }
        }
        Ok(())
    }

    pub fn evaluate(&mut self, stmt: &Stmt) -> Result<(), Error> {
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
                    self.env.define(sym, Value::Undefined);
                    return Ok(());
                }
            },
            Stmt::Block(statements) => {
                let environment = Environment::with_enclosing(self.env.clone());
                return self.interpret_block(statements, environment);
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
            Stmt::FunDecl(fun) => {
                let func = LoxFunction {
                    declaration: Stmt::FunDecl(fun.clone()),
                };
                self.env.define(&fun.name, Value::LoxFunction(func));
                return Ok(());
            }
            Stmt::Return(_, expr) => {
                let mut value = Value::Nil;
                if let Some(expr) = expr {
                    value = self.get_value(expr)?;
                }
                self.ret_val = Some(value.clone());
                return Err(Error::Return { value });
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
                let value = self.env.get(&v)?;
                if let Value::Undefined = value {
                    return Err(Error::UndefinedVariable {
                        name: v.name.clone(),
                        line: v.line,
                        col: v.col,
                    });
                }
                return self.env.get(&v);
            }
            Expr::Logical(left, op, right) => {
                return self.interpret_logical(left, op, right);
            }
            Expr::Call(callee, location, args) => {
                let callee = self.get_value(callee)?;
                let mut arguments = Vec::new();
                for arg in args {
                    arguments.push(self.get_value(arg)?);
                }
                match callee {
                    Value::NativeFunction(callee) => {
                        let res = callee.call(self, arguments.as_slice());
                        match res {
                            Ok(v) => Ok(v),
                            Err(e) => Err(Error::JustError { message: e }),
                        }
                    }
                    Value::LoxFunction(callee) => {
                        let res = callee.call(self, arguments.as_slice());
                        match res {
                            Ok(v) => Ok(v),
                            Err(e) => Err(Error::JustError { message: e }),
                        }
                    }
                    _ => {
                        return Err(Error::RunTimeException {
                            message: "Can only call function and classes".to_string(),
                            line: location.line,
                            col: location.col,
                        })
                    }
                }
            }
            _ => return Ok(Value::Nil),
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

    pub fn interpret_block(&mut self, statements: &[Stmt], env: Environment) -> Result<(), Error> {
        self.env = env;
        for statement in statements {
            if let Stmt::Break = statement {
                self.loop_stack.pop();
                break;
            }
            let res = self.evaluate(statement);
            if let Err(e) = res {
                return Err(e);
            }
        }
        if let Some(enclosing) = self.env.enclosing.clone() {
            self.env = *enclosing;
        }
        return Ok(());
    }
}

fn is_truthy(val: &Value) -> Result<bool, Error> {
    match val {
        Value::Number(v) => {
            if *v > 0.0 {
                return Ok(true);
            }
            return Ok(false);
        }
        Value::Bool(v) => {
            return Ok(*v);
        }
        _ => {
            return Err(Error::JustError {
                message: String::from("{val} cannot be casted to bool"),
            })
        }
    }
}
