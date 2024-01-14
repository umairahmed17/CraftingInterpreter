use crate::token::Token;

#[macro_export]
macro_rules! paranthesize {
    ($($args:expr),*) => {
        {
            let mut out = String::from("(");
            $(
               out += &$args.to_string();
             )*
            out += ")";
            out
        }
    };
}

#[derive(Debug)]
pub enum Expression<'a> {
    Binary(Binary<'a>),
    Grouping(Grouping<'a>),
    Literal(usize),
    Unary(Unary<'a>),
}

// pub trait Expression {
//     fn expr(&self) -> &'static str {
//         return "I am an expression";
//     }
// }

impl<'a> ToString for Expression<'a> {
    fn to_string(&self) -> String {
        match self {
            Expression::Binary(_) => return "binary".to_string(),
            Expression::Grouping(_) => return "grouping".to_string(),
            Expression::Literal(usize) => return "literal".to_string(),
            Expression::Unary(_) => return "unary".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Binary<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: Token<'a>,
    pub right: Box<Expression<'a>>,
}

#[derive(Debug)]
pub struct Grouping<'a> {
    pub expression: Box<Expression<'a>>,
}

#[derive(Debug)]
pub struct Unary<'a> {
    pub operator: Token<'a>,
    pub right: Box<Expression<'a>>,
}

pub trait PrettyPrint {
    fn print(&self) -> String;
}

impl<'a> PrettyPrint for Expression<'a> {
    fn print(&self) -> String {
        return self.print();
    }
}

impl<'a> PrettyPrint for Binary<'a> {
    fn print(&self) -> String {
        return paranthesize!(&self.operator, &self.left, &self.right);
    }
}

impl<'a> PrettyPrint for Grouping<'a> {
    fn print(&self) -> String {
        return paranthesize!(&self.expression);
    }
}

impl<'a> PrettyPrint for Unary<'a> {
    fn print(&self) -> String {
        return paranthesize!(&self.operator, &self.right);
    }
}
