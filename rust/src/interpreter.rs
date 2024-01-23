use crate::{expr::Stmt, error::Error};

pub fn interpret(stmts: Vec<Stmt>) -> Result<(), Error> {
    for stmt in stmts {
        if let Err(res) = evaluate(stmt){
            return Err(res);
        }
    }
    Ok(())
}

fn evaluate(stmt: Stmt) -> Result<(), Error> {
    match stmt {
        Stmt::Expr(v) => {
            match v.get_value() {
                Ok(v) => {
                    println!("{v:?}");
                    return Ok(());
                },
                Err(e) => return Err(e),
            }
        },
        Stmt::Print(v) => {
            match v.get_value() {
                Ok(v) => {
                    println!("{v:?}");
                    return Ok(());
                },
                Err(e) => return Err(e),
            }
        },
        _ => return Ok(())
    }
}
