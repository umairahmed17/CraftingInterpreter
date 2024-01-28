mod env;
mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;

use std::collections::HashMap;

use clap::Parser;
use env::Environment;
use parser::LoxParser;
use scanner::{scan_tokens, Token};

use crate::interpreter::Interpreter;

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
    let mut environment = Environment {
        values: HashMap::new(),
        enclosing: None,
    };

    loop {
        println!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let tokens = scan_tokens(input.clone());
        match tokens {
            Ok(tokens) => {
                let mut parser = LoxParser { tokens, current: 0 };
                let stmts = parser.parse().unwrap();
                let mut interpreter = Interpreter {
                    statements: &stmts,
                    env: environment.clone(),
                };
                if let Err(e) = interpreter.interpret() {
                    println!("{e:?}");
                    return;
                }
                environment = interpreter.env;
            }
            Err(e) => print!("{e:?}\n"),
        }
    }
}

fn run(content: String) {
    let tokens = scan_tokens(content);
    match tokens {
        Ok(tokens) => {
            let mut parser = LoxParser { tokens, current: 0 };
            let stmts = parser.parse().unwrap();
            let mut interpreter = Interpreter {
                statements: &stmts,
                env: Environment {
                    values: HashMap::new(),
                    enclosing: None,
                },
            };
            if let Err(e) = interpreter.interpret() {
                println!("{e:?}");
                return;
            }
        }
        Err(e) => print!("{e:?}\n"),
    }
}
