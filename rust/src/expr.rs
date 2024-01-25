use core::{fmt, panic};
use std::str::FromStr;

use crate::{error::Error, scanner::Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    // This(SourceLocation),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    // Call(Box<Expr>, SourceLocation, Vec<Expr>),
    // Get(Box<Expr>, Symbol),
    Grouping(Box<Expr>),
    Variable(Symbol),
    // Assign(Symbol, Box<Expr>),
    // Logical(Box<Expr>, LogicalOp, Box<Expr>),
    // Set(Box<Expr>, Symbol, Box<Expr>),
    // Super(SourceLocation, Symbol),
    // List(Vec<Expr>),
    // Subscript {
    //     value: Box<Expr>,
    //     slice: Box<Expr>,
    //     source_location: SourceLocation,
    // },
    // SetItem {
    //     lhs: Box<Expr>,
    //     slice: Box<Expr>,
    //     rhs: Box<Expr>,
    //     source_location: SourceLocation,
    // },
    // Lambda(LambdaDecl),
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(v) => write!(f, "{}", v),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "Nil"),
        }
    }
}

impl Expr {
    pub fn get_value(&self) -> Result<Value, Error> {
        match self {
            Expr::Literal(val) => match val {
                Literal::Number(val) => Ok(Value::Number(*val)),
                Literal::String(val) => Ok(Value::String(val.to_string())),
                Literal::True => Ok(Value::Bool(true)),
                Literal::False => Ok(Value::Bool(true)),
                Literal::Nil => Ok(Value::Nil),
            },
            Expr::Unary(op, expr) => {
                let right: Value = expr.get_value()?;
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
            Expr::Binary(left_expr, op, right_expr) => {
                let right = right_expr.get_value()?;
                let left = left_expr.get_value()?;

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
                                let message = "Wrong Binary Token In {op:?} with {right_expr:?} and {left_expr:?}";
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
                                let message = "Wrong Binary Token In {op:?} with {right_expr:?} and {left_expr:?}";
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
                                let message = "Wrong Binary Token In {op:?} with {right_expr:?} and {left_expr:?}";
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
            Expr::Grouping(expr) => return expr.get_value(),
            _ => return Err(Error::JustError)
        }
    }

}

fn is_truthy(v: f64) -> bool {
    if v > 0.0 {
        return true;
    }
    return false;
}

#[derive(Debug, Clone, Copy)]
pub struct SourceLocation {
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    Or,
    And,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Symbol {
    pub name: String,
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Clone)]
pub struct FunDecl {
    pub name: Symbol,
    pub params: Vec<Symbol>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct LambdaDecl {
    pub params: Vec<Symbol>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub name: Symbol,
    pub superclass: Option<Symbol>,
    pub methods: Vec<FunDecl>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    FunDecl(FunDecl),
    ClassDecl(ClassDecl),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    VarDecl(Symbol, Option<Expr>),
    Block(Vec<Stmt>),
    Return(SourceLocation, Option<Expr>),
    While(Expr, Box<Stmt>),
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOpTy {
    Minus,
    Bang,
}

#[derive(Debug, Copy, Clone)]
pub struct UnaryOp {
    pub ty: UnaryOpTy,
    pub line: usize,
    pub col: i64,
}

impl UnaryOp {
    pub(crate) fn from_token(current: &Token) -> UnaryOp {
        return match current.ty {
            crate::scanner::TokenType::Minus => Self {
                ty: UnaryOpTy::Minus,
                line: current.line,
                col: current.col,
            },
            crate::scanner::TokenType::Bang => Self {
                ty: UnaryOpTy::Bang,
                line: current.line,
                col: current.col,
            },
            _ => panic!(
                "this was not supposed to happen! This token `{current:?}` is not a `BinaryOpTy`"
            ),
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOpTy {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Copy, Clone)]
pub struct BinaryOp {
    pub ty: BinaryOpTy,
    pub line: usize,
    pub col: i64,
}

impl BinaryOp {
    pub fn from_token(value: &Token) -> Self {
        match value.ty {
            crate::scanner::TokenType::Minus => Self {
                ty: BinaryOpTy::Minus,
                line: value.line,
                col: value.col,
            },
            crate::scanner::TokenType::Plus => Self {
                ty: BinaryOpTy::Plus,
                line: value.line,
                col: value.col,
            },
            crate::scanner::TokenType::Slash => Self {
                ty: BinaryOpTy::Slash,
                line: value.line,
                col: value.col,
            },
            crate::scanner::TokenType::Star => Self {
                ty: BinaryOpTy::Star,
                line: value.line,
                col: value.col,
            },
            crate::scanner::TokenType::BangEqual => Self {
                ty: BinaryOpTy::NotEqual,
                line: value.line,
                col: value.col,
            },
            crate::scanner::TokenType::EqualEqual => Self {
                ty: BinaryOpTy::EqualEqual,
                line: value.line,
                col: value.col,
            },
            crate::scanner::TokenType::Greater => Self {
                ty: BinaryOpTy::Greater,
                line: value.line,
                col: value.col,
            },
            crate::scanner::TokenType::GreaterEqual => Self {
                ty: BinaryOpTy::GreaterEqual,
                line: value.line,
                col: value.col,
            },
            crate::scanner::TokenType::Less => Self {
                ty: BinaryOpTy::Less,
                line: value.line,
                col: value.col,
            },
            crate::scanner::TokenType::LessEqual => Self {
                ty: BinaryOpTy::LessEqual,
                line: value.line,
                col: value.col,
            },
            _ => panic!(
                "this was not supposed to happen! This token `{value:?}` is not a `BinaryOpTy`"
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}
