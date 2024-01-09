use crate::error::{self, error};

#[derive(Debug, Clone, Copy)]
enum TokenType<'a> {
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
    Identifier,
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

#[derive(Debug)]
pub struct Token<'a> {
    token_type: TokenType<'a>,
    line: usize,
}

impl<'a> Token<'a> {
    fn generate_token(token_type: TokenType<'a>, line: usize) -> Self {
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
    pub current: usize,
    pub start: usize,
    pub line: usize,
}

impl<'a> Lexer<'a> {
    pub fn scan_content(&'a mut self) -> &Vec<Token> {
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
                    while self.peek() != '\n' && !self.is_at_end() {
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
                if self.peek().is_digit(10) {
                    self.number();
                } else if self.peek().is_alphabetic() {
                    self.identifier();
                } else {
                    error(self.line, "Unexpected character");
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.source.chars().collect::<Vec<char>>();
        self.current += 1;
        return *ch.get(self.current - 1).unwrap();
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
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.tokens.push(Token::generate_token(
            TokenType::Number(
                self.source[self.start..self.current]
                    .parse::<usize>()
                    .unwrap(),
            ),
            self.line,
        ));
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().next().unwrap();
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
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
        while self.peek().is_alphabetic() {
            self.advance();
        }
        let text: &str = &self.source[self.start..self.current];
        let token_type = KEYWORDS.iter().find(|(x, _)| *x == text);
        match token_type {
            Some((_, token_type)) => self
                .tokens
                .push(Token::generate_token(*token_type, self.line)),
            None => self
                .tokens
                .push(Token::generate_token(TokenType::Identifier, self.line)),
        }
    }
}
