use crate::token::Token;
use crate::token_type::{match_keyword, TokenType};

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    pub tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: i32,

    has_error: bool,
    error_lines: Vec<i32>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
            error_lines: Vec::new(),
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        });
    }

    fn error(&mut self, _message: &str, line: i32) {
        self.has_error = true;
        self.error_lines.push(line);
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '/' => {
                if !self.match_next('/') {
                    self.add_token(TokenType::Slash)
                }
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '"' => self.string(),
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    self.error("Unexpected character", self.line);
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphabetic() {
            self.advance();
        }
        let identifier = &self.source[self.start..self.current];
        match match_keyword(identifier) {
            Some(token_type) => self.add_token(token_type),
            None => self.add_token(TokenType::Identifier),
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        // it is okay to unwrap here because we this should not be called when we are at the end
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literal: "".to_string(),
            line: self.line,
        });
    }

    fn add_token_and_literal(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literal,
            line: self.line,
        });
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
        }

        self.add_token_and_literal(
            TokenType::Number,
            self.source[self.start..self.current].to_string(),
        );
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            if self.is_at_end() {
                self.error("Unterminated string", self.line);
                return;
            }
            self.advance();
        }
    }
}
