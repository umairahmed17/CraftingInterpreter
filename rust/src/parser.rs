use std::rc::Rc;

use crate::{expr::Expr, scanner::TokenType, Token};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn expr(&self) -> Expr {
        return self.equality();
    }

    fn equality(&self) -> Expr {
        let mut expr: Expr = self.comparision();
        while self.match_one_of(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op: Token = self.tokens[self.current - 1];
            let right: Expr = self.comparision();
            expr = Expr::Binary(expr, op, right);
        }

        return expr;
    }

    fn match_one_of(&self, tokens: Vec<TokenType>) -> bool {
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
        return self.tokens[self.current - 1];
    }

    fn is_at_end(&self) -> bool {
        return self.peek().ty == TokenType::Eof;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current];
    }
}
