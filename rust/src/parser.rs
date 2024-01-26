use crate::{
    error::Error,
    expr::{BinaryOp, Expr, Literal, Stmt, Symbol, UnaryOp},
    scanner::{self, TokenType},
    Token,
};

pub struct LoxParser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl LoxParser {
    fn expr(&mut self) -> Result<Expr, Error> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.equality()?;
        if self.match_one_of(vec![TokenType::Equal]) {
            let equals = &self.tokens[self.current - 1].clone(); // idk what i am doing
            let value = self.assignment()?;

            if let Expr::Variable(v) = expr {
                return Ok(Expr::Assign(v, Box::new(value)));
            }

            return Err(Error::InvalidAssignment {
                line: equals.line,
                col: equals.col,
            });
        }
        return Ok(expr);
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.comparision()?;
        while self.match_one_of(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op: BinaryOp = BinaryOp::from_token(&self.tokens[self.current - 1]);
            let right: Expr = self.comparision()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        return Ok(expr);
    }

    fn match_one_of(&mut self, tokens: Vec<TokenType>) -> bool {
        for token in tokens {
            if self.check_type(token) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check_type(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().ty == token;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.tokens[self.current - 1].clone();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().ty == TokenType::Eof;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn comparision(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.term()?;

        while self.match_one_of(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op: BinaryOp = BinaryOp::from_token(&self.tokens[self.current - 1]);
            let right: Expr = self.term()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.factor()?;

        while self.match_one_of(vec![TokenType::Minus, TokenType::Plus]) {
            let op: BinaryOp = BinaryOp::from_token(&self.tokens[self.current - 1]);
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right))
        }
        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.unary()?;

        while self.match_one_of(vec![TokenType::Slash, TokenType::Star]) {
            let op: BinaryOp = BinaryOp::from_token(&self.tokens[self.current - 1]);
            let right: Expr = self.unary()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right))
        }
        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_one_of(vec![TokenType::Bang, TokenType::Minus]) {
            let op: UnaryOp = UnaryOp::from_token(&self.tokens[self.current - 1]);
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary(op, Box::new(right)));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_one_of(vec![TokenType::False]) {
            return Ok(Expr::Literal(crate::expr::Literal::False));
        }
        if self.match_one_of(vec![TokenType::True]) {
            return Ok(Expr::Literal(crate::expr::Literal::True));
        }
        if self.match_one_of(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(crate::expr::Literal::Nil));
        }
        if self.match_one_of(vec![TokenType::Number, TokenType::String]) {
            let prev = &self.tokens[self.current - 1].literal;
            match &prev {
                Some(scanner::Literal::Number(n)) => return Ok(Expr::Literal(Literal::Number(*n))),
                Some(scanner::Literal::Str(s)) => {
                    return Ok(Expr::Literal(Literal::String(s.clone())))
                }
                Some(l) => panic!(
                    "internal error in parser: when parsing number, found literal {:?}",
                    l
                ),
                None => panic!("internal error in parser: when parsing number, found no literal"),
            }
        }
        if self.match_one_of(vec![TokenType::Identifier]) {
            let name = &self.tokens[self.current - 1];
            let symbol_name = String::from_utf8(name.lexeme.clone()).unwrap();
            let name = Symbol {
                name: symbol_name,
                line: name.line,
                col: name.col,
            };
            return Ok(Expr::Variable(name));
        }
        if self.match_one_of(vec![TokenType::LeftParen]) {
            let expr: Expr = self.expr()?;
            let _ = self.consume(TokenType::RightParen, "Expect `)` after expression.");
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        return Err(Error::ExpectedExpression {
            token_type: self.peek().ty,
            line: self.peek().line,
            col: self.peek().col,
        });
    }

    fn consume(&mut self, tok: TokenType, on_err_str: &str) -> Result<Token, Error> {
        if self.check_type(tok) {
            return Ok(self.advance());
        }
        return Err(Error::TokenMismatch {
            expected: tok,
            found: self.peek().clone(),
            maybe_on_err_string: Some(on_err_str.into()),
        });
    }

    fn synchronize(&mut self) -> () {
        self.advance();
        while !self.is_at_end() {
            let prev = &self.tokens[self.current - 1];

            if prev.ty == TokenType::Semicolon {
                return;
            }

            match self.peek().ty {
                TokenType::Semicolon => return,
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        return Ok(statements);
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.match_one_of(vec![TokenType::Print]) {
            return self.print_statement();
        }
        return self.expr_statement();
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value: Expr = self.expr().unwrap();
        if let Err(res) = self.consume(TokenType::Semicolon, "Expect `;` after value") {
            return Err(res);
        }
        return Ok(Stmt::Print(value));
    }

    fn expr_statement(&mut self) -> Result<Stmt, Error> {
        let value: Expr = self.expr().unwrap();
        if let Err(res) = self.consume(TokenType::Semicolon, "Expect `;` after expression") {
            return Err(res);
        }
        return Ok(Stmt::Expr(value));
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        if self.match_one_of(vec![TokenType::Var]) {
            let stmt = self.var_declaration();
            if let Err(_) = stmt {
                self.synchronize();
            }
            return stmt;
        }
        let stmt = self.statement();
        if let Err(_) = stmt {
            self.synchronize();
        }
        return stmt;
    }

    fn var_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(TokenType::Identifier, "Expect variable name")?;
        let symbol_name = String::from_utf8(name.lexeme).unwrap();
        let name = Symbol {
            name: symbol_name,
            line: name.line,
            col: name.col,
        };
        let mut initializer: Option<Expr> = None;
        if self.match_one_of(vec![TokenType::Equal]) {
            initializer = Some(self.expr()?);
        }
        let _ = self.consume(
            TokenType::Semicolon,
            "Expect `;` after variable declaration",
        );
        return Ok(Stmt::VarDecl(name, initializer));
    }
}
