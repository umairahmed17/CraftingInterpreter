use core::{fmt, panic};
use std::str::FromStr;

use crate::{env::Environment, error::Error, scanner::Token};

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
    Assign(Symbol, Box<Expr>),
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
    Assign(Symbol, Box<Stmt>),
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
