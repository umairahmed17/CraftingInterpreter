mod error;
mod expr;
mod parser;
mod scanner;

use clap::Parser;
use error::Error;
use expr::{BinaryOp, Expr, Literal};
use parser::LoxParser;
use scanner::{scan_tokens, Token};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Arguments::parse();
    match args.file {
        Some(file) => run_file(&file),
        None => run_prompt(),
    }
}

fn run_file(file: &str) {
    let content = std::fs::read_to_string(file).expect("Failed to read file");
    run(content);
}

fn run_prompt() {
    loop {
        println!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        run(input);
    }
}

fn run(content: String) {
    let tokens = scan_tokens(content);
    match tokens {
        Ok(tokens) => {
            let mut parser = LoxParser { tokens, current: 0 };
            match parser.parse() {
                Ok(expr) => {
                    println!("{expr:?}");
                    Expr::interpret(expr);
                },
                Err(err) => {
                    println!("{err:?}");
                }
            }
        }
        Err(e) => print!("{e:?}\n"),
    }
}
