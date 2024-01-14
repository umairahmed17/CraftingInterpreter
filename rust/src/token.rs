use std::{fmt::Display, iter::Peekable, str::Chars};

use crate::error::error;

#[derive(Debug, Clone, Copy)]
pub enum TokenType<'a> {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(&'a str),
    QuotedString(&'a str),
    Number(usize),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl<'a> Display for TokenType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => {
                return write!(f, "left paren");
            }
            TokenType::RightParen => {
                return write!(f, "right paren");
            }
            TokenType::LeftBrace => {
                return write!(f, "left brace");
            }
            TokenType::RightBrace => {
                return write!(f, "right brace");
            }
            TokenType::Comma => {
                return write!(f, "comma");
            }
            TokenType::Dot => {
                return write!(f, "dot");
            }
            TokenType::Minus => {
                return write!(f, "minus");
            }
            TokenType::Plus => {
                return write!(f, "plus");
            }
            TokenType::Semicolon => {
                return write!(f, "semicolon");
            }
            TokenType::Slash => {
                return write!(f, "slash");
            }
            TokenType::Star => {
                return write!(f, "star");
            }
            TokenType::Bang => {
                return write!(f, "bang");
            }
            TokenType::BangEqual => {
                return write!(f, "BangEqual");
            }
            TokenType::Equal => {
                return write!(f, "Equal");
            }
            TokenType::EqualEqual => {
                return write!(f, "EqualEqual");
            }
            TokenType::Greater => {
                return write!(f, "Greater");
            }
            TokenType::GreaterEqual => {
                return write!(f, "GreaterEqual");
            }
            TokenType::Less => {
                return write!(f, "Less");
            }
            TokenType::LessEqual => {
                return write!(f, "LessEqual");
            }
            TokenType::Identifier(_) => {
                return write!(f, "Identifie");
            }
            TokenType::QuotedString(_) => {
                return write!(f, "QuotedStrin");
            }
            TokenType::Number(_) => {
                return write!(f, "Numbe");
            }
            TokenType::And => {
                return write!(f, "And");
            }
            TokenType::Class => {
                return write!(f, "Class");
            }
            TokenType::Else => {
                return write!(f, "Else");
            }
            TokenType::False => {
                return write!(f, "False");
            }
            TokenType::Fun => {
                return write!(f, "Fun");
            }
            TokenType::For => {
                return write!(f, "For");
            }
            TokenType::If => {
                return write!(f, "If");
            }
            TokenType::Nil => {
                return write!(f, "Nil");
            }
            TokenType::Or => {
                return write!(f, "Or");
            }
            TokenType::Print => {
                return write!(f, "Print");
            }
            TokenType::Return => {
                return write!(f, "Return");
            }
            TokenType::Super => {
                return write!(f, "Super");
            }
            TokenType::This => {
                return write!(f, "This");
            }
            TokenType::True => {
                return write!(f, "True");
            }
            TokenType::Var => {
                return write!(f, "Var");
            }
            TokenType::While => {
                return write!(f, "While");
            }
            TokenType::EOF => {
                return write!(f, "EOF");
            }
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    token_type: TokenType<'a>,
    line: usize,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token {{ token_type: {0}, line: {1} }}",
            self.token_type, self.line
        );
        return Ok(());
    }
}

impl<'a> Token<'a> {
    pub fn generate_token(token_type: TokenType<'a>, line: usize) -> Self {
        return Token { token_type, line };
    }
}

const KEYWORDS: [(&str, TokenType); 16] = [
    ("while", TokenType::While),
    ("var", TokenType::Var),
    ("true", TokenType::True),
    ("this", TokenType::This),
    ("super", TokenType::Super),
    ("return", TokenType::Return),
    ("print", TokenType::Print),
    ("or", TokenType::Or),
    ("nil", TokenType::Nil),
    ("if", TokenType::If),
    ("for", TokenType::For),
    ("fn", TokenType::Fun),
    ("false", TokenType::False),
    ("else", TokenType::Else),
    ("class", TokenType::Class),
    ("and", TokenType::And),
];

#[derive(Debug)]
pub struct Lexer<'a> {
    pub tokens: Vec<Token<'a>>,
    pub source: &'a str,
    pub iter: Peekable<Chars<'a>>,
    pub current: usize,
    pub start: usize,
    pub line: usize,
}

impl<'a> Lexer<'a> {
    pub fn scan_content(&'a mut self) -> &Vec<Token> {
        println!("The length of source is {0}", self.source.len());
        return self.scan_tokens();
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::generate_token(TokenType::EOF, self.line));
        return &self.tokens;
    }

    fn is_at_end(&mut self) -> bool {
        return self.current >= self.source.len();
    }

    fn scan_token(&mut self) -> () {
        let c = self.advance();
        println!(
            "The current character at {0} to be tokenize is: {c}",
            self.current
        );
        match c {
            '(' => self
                .tokens
                .push(Token::generate_token(TokenType::LeftParen, self.line)),
            ')' => self
                .tokens
                .push(Token::generate_token(TokenType::RightParen, self.line)),
            '{' => self
                .tokens
                .push(Token::generate_token(TokenType::LeftBrace, self.line)),
            '}' => self
                .tokens
                .push(Token::generate_token(TokenType::RightBrace, self.line)),
            ',' => self
                .tokens
                .push(Token::generate_token(TokenType::Comma, self.line)),
            '.' => self
                .tokens
                .push(Token::generate_token(TokenType::Dot, self.line)),
            '-' => self
                .tokens
                .push(Token::generate_token(TokenType::Minus, self.line)),
            '+' => self
                .tokens
                .push(Token::generate_token(TokenType::Plus, self.line)),
            ';' => self
                .tokens
                .push(Token::generate_token(TokenType::Semicolon, self.line)),
            '*' => self
                .tokens
                .push(Token::generate_token(TokenType::Star, self.line)),
            '!' => {
                if self.r#match('=') {
                    return self
                        .tokens
                        .push(Token::generate_token(TokenType::BangEqual, self.line));
                }
                return self
                    .tokens
                    .push(Token::generate_token(TokenType::BangEqual, self.line));
            }
            '=' => {
                if self.r#match('=') {
                    return self
                        .tokens
                        .push(Token::generate_token(TokenType::EqualEqual, self.line));
                }
                return self
                    .tokens
                    .push(Token::generate_token(TokenType::Equal, self.line));
            }
            '<' => {
                if self.r#match('=') {
                    return self
                        .tokens
                        .push(Token::generate_token(TokenType::LessEqual, self.line));
                }
                return self
                    .tokens
                    .push(Token::generate_token(TokenType::Less, self.line));
            }
            '>' => {
                if self.r#match('=') {
                    return self
                        .tokens
                        .push(Token::generate_token(TokenType::GreaterEqual, self.line));
                }
                return self
                    .tokens
                    .push(Token::generate_token(TokenType::Greater, self.line));
            }
            '/' => {
                if self.r#match('/') {
                    while *self.iter.peek().unwrap_or(&'\0') != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                return self
                    .tokens
                    .push(Token::generate_token(TokenType::Slash, self.line));
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.handle_string(),
            _ => {
                if c.is_numeric() {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    // println!("Maybe a number {c:?}: {0}", c.is_numeric());
                    error(self.line, "Unexpected character");
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.iter.next().unwrap_or('\0');
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.advance() != expected {
            return false;
        }

        return true;
    }

    fn number(&mut self) {
        while self.iter.peek().unwrap_or(&'\0').is_numeric() {
            self.advance();
        }

        // if *self.iter.peek().unwrap_or(&'\0') == '.' && self.iter.nth(self.current + 1).unwrap_or('\0').is_numeric()
        // {
        //     self.advance();
        //     while self.iter.peek().unwrap_or(&'\0').is_numeric() {
        //         self.advance();
        //     }
        // }

        let text = &self.source[self.start..self.current];
        println!("The Number Text is: {text:?}");
        self.tokens.push(Token::generate_token(
            TokenType::Number(
                self.source[self.start..self.current]
                    .parse::<usize>()
                    .unwrap(),
            ),
            self.line,
        ));
    }

    fn handle_string(&mut self) {
        while *self.iter.peek().unwrap_or(&'\0') != '"' && !self.is_at_end() {
            if *self.iter.peek().unwrap_or(&'\0') == '\n' {
                self.line += self.line;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.");
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.tokens.push(Token::generate_token(
            TokenType::QuotedString(&value),
            self.line,
        ));
    }

    fn identifier(&mut self) {
        loop {
            let c = self.iter.peek().unwrap_or(&'\0');
            if c.is_ascii_uppercase() || c.is_ascii_lowercase() || *c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let text: &str = &self.source[self.start..self.current];
        let token_type = KEYWORDS.iter().find(|(x, _)| *x == text);
        match token_type {
            Some((_, token_type)) => self
                .tokens
                .push(Token::generate_token(*token_type, self.line)),
            None => self.tokens.push(Token::generate_token(
                TokenType::Identifier(text),
                self.line,
            )),
        }
    }
}
