#[derive(Debug)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
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
    String,
    Number,

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

    Eof,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: i32,
}

struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,

    start: i32,
    current: i32,
    line: i32,
}

impl<'a> Scanner<'a> {
    fn new(source: &str) -> Scanner {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }

    fn scan_tokens(&mut self) {
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

    fn scan_token(&mut self) {}

    fn advance(&mut self) -> char {
        self.current += 1;
        // it is okay to unwrap here because we this should not be called when we are at the end
        self.source.chars().nth(self.current as usize - 1).unwrap();
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text.to_string(),
            literal: "".to_string(),
            line: self.line,
        });
    }

    fn add_token_and_literal(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text.to_string(),
            literal: literal,
            line: self.line,
        });
    }
}
