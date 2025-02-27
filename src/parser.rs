use crate::expr::{Expr, Value};
use crate::token::Token;
use crate::token_type::TokenType;
use crate::parser_error::ParserError;
use std::cell::Cell;

pub struct Parser<'a> {
    tokens: &'a [Token],
    pub current: Cell<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Parser<'a> {
        Parser {
            tokens,
            current: Cell::new(0),
        }
    }

    pub fn parse(&self) -> Result<Box<Expr>, ParserError> {
        return self.expression();
    }

    fn expression(&self) -> Result<Box<Expr>, ParserError> {
        return self.equality()
    }

    fn equality(&self) -> Result<Box<Expr>, ParserError> {
        let mut expr = self.comparison()?;
        while self.match_next(TokenType::BangEqual) || self.match_next(TokenType::EqualEqual) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        Ok(expr)
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

    fn comparison(&self) -> Result<Box<Expr>, ParserError> {
        let mut expr = self.term()?;
        while self.match_next(TokenType::Greater)
            || self.match_next(TokenType::GreaterEqual)
            || self.match_next(TokenType::Less)
            || self.match_next(TokenType::LessEqual)
        {
            let operator = self.previous();
            let right = self.term()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        Ok(expr)
    }

    fn term(&self) ->Result<Box<Expr>, ParserError> {
        let mut expr     = self.factor()?;
        while self.match_next(TokenType::Minus) || self.match_next(TokenType::Plus) {
            let operator = self.previous();
            let right =    self.factor()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
Ok(expr)
    }

    fn factor(&self) -> Result<Box<Expr>, ParserError> {
        let mut expr: Box<Expr> = self.unary()?;
        while self.match_next(TokenType::Slash) || self.match_next(TokenType::Star) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    fn unary(&self) -> Result<Box<Expr>, ParserError> {
        if self.match_next(TokenType::Minus) || self.match_next(TokenType::Bang) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary(operator, right)));
        }
        return self.primary();
    }

    fn primary(&self) -> Result<Box<Expr>, ParserError> {
        if self.match_next(TokenType::False) {
            return Ok(Box::new(Expr::Literal(Value::Boolean(false))));
        }
        if self.match_next(TokenType::True) {
            return Ok(Box::new(Expr::Literal(Value::Boolean(true))));
        }
        if self.match_next(TokenType::Nil) {
            return Ok(Box::new(Expr::Literal(Value::Nil)));
        }
        if self.match_next(TokenType::String) {
            return Ok(Box::new(Expr::Literal(Value::String(
                self.previous().literal.clone(),
            ))));
        }

        if self.match_next(TokenType::Number) {
            return Ok(Box::new(Expr::Literal(Value::Number(
                self.previous().literal.clone(),
            ))));
        }

        if self.match_next(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Ok(Box::new(Expr::Grouping(expr)));
        }
        Err(ParserError::Error("No matching clause in primary".to_string()))
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
