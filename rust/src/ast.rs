use crate::grammar::{Expression, PrettyPrint};

pub struct AST<'a> {
    pub expr: Expression<'a>,
}

impl<'a> AST<'a> {
    pub fn print(&self) -> String {
        return self.expr.print();
    }
}
