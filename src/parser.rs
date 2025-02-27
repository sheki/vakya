use crate::expr::{Expr, Value};
use crate::token::Token;
use crate::token_type::TokenType;
use std::cell::Cell;

struct Parser<'a> {
    tokens: &'a [Token],
    current: Cell<usize>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Parser<'a> {
        Parser {
            tokens,
            current: Cell::new(0),
        }
    }

    fn expression(&self) -> Box<Expr> {
        return self.equality();
    }

    fn equality(&self) -> Box<Expr> {
        let mut expr = self.comparison();
        while self.match_next(TokenType::BangEqual) || self.match_next(TokenType::EqualEqual) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        expr
    }

    fn previous(&self) -> &Token {
        let current = self.current.get();
        &self.tokens[current - 1]
    }

    fn match_next(&self, expected: TokenType) -> bool {
        if self.current.get() >= self.tokens.len() {
            return false;
        }
        let current = self.current.get();
        if self.tokens[current].token_type == TokenType::Eof {
            return false;
        }
        if self.tokens[current].token_type != expected {
            return false;
        }
        self.current.set(current + 1);
        return true;
    }

    fn comparison(&self) -> Box<Expr> {
        let mut expr = self.term();
        while self.match_next(TokenType::Greater)
            || self.match_next(TokenType::GreaterEqual)
            || self.match_next(TokenType::Less)
            || self.match_next(TokenType::LessEqual)
        {
            let operator = self.previous();
            let right = self.term();
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        expr
    }

    fn term(&self) -> Box<Expr> {
        let mut expr = self.factor();
        while self.match_next(TokenType::Minus) || self.match_next(TokenType::Plus) {
            let operator = self.previous();
            let right = self.factor();
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        expr
    }

    fn factor(&self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.unary();
        while self.match_next(TokenType::Slash) || self.match_next(TokenType::Star) {
            let operator = self.previous();
            let right = self.unary();
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        expr
    }

    fn unary(&self) -> Box<Expr> {
        if self.match_next(TokenType::Minus) || self.match_next(TokenType::Bang) {
            let operator = self.previous();
            let right = self.unary();
            return Box::new(Expr::Unary(operator, right));
        }
        return self.primary();
    }

    fn primary(&self) -> Box<Expr> {
        if self.match_next(TokenType::False) {
            return Box::new(Expr::Literal(Value::Boolean(false)));
        }
        if self.match_next(TokenType::True) {
            return Box::new(Expr::Literal(Value::Boolean(true)));
        }
        if self.match_next(TokenType::Nil) {
            return Box::new(Expr::Literal(Value::Nil));
        }
        if self.match_next(TokenType::String) {
            return Box::new(Expr::Literal(Value::String(
                self.previous().literal.clone(),
            )));
        }
        if self.match_next(TokenType::Number) {
            return Box::new(Expr::Literal(Value::Number(
                self.previous().literal.clone(),
            )));
        }

        if self.match_next(TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Box::new(Expr::Grouping(expr));
        }
        panic!("No matching clause in primary")
    }

    fn consume(&self, expected: TokenType, message: &str) {
        let current = self.current.get();
        if current >= self.tokens.len() {
            panic!("error ${message}")
        }
        if self.tokens[current].token_type == TokenType::Eof {
            panic!("error ${message}")
        }
        if self.tokens[current].token_type != expected {
            panic!("error ${message}")
        }
        self.current.set(current + 1);
    }
}
