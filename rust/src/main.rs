use clap::Parser;
mod error;
mod token;

static mut HAD_ERROR: bool = false;

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
    unsafe {
        if HAD_ERROR {
            std::process::exit(65);
        }
    }
}

fn run_prompt() {
    loop {
        println!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        run(input);
        unsafe {
            HAD_ERROR = false;
        }
    }
}

fn run(content: String) {
    let tokens: Vec<token::Token> = token::Token::scan_content(&content);
    println!("{tokens:?}");
}
