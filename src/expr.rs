#[derive(Debug)]
enum  Expr {
    Binary(Box<Expr>, Token, Box<Expr>), 
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(left, op, right) => write!(f, "({} {} {})", left, op, right),
            Expr::Grouping(expr) => write!(f, "({})", expr),
            Expr::Literal(token) => write!(f, "{}", token.lexeme),
            Expr::Unary(op, expr) => write!(f, "({} {})", op, expr),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_to_string() {
        assert_eq!(Expr::Literal(Token {
            token_type: TokenType::Number,
            lexeme: "42".to_string(),
            literal: "42".to_string(),
            line: 1,
        }).to_string(), "42");

        assert_eq!(Expr::Unary(Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: "".to_string(),
            line: 1,
        }, Box::new(Expr::Literal(Token {
            token_type: TokenType::Number,
            lexeme: "42".to_string(),
            literal: "42".to_string(),
            line: 1,
        }))).to_string(), "(- 42)");

        assert_eq!(Expr::Binary(Box::new(Expr::Literal(Token {
            token_type: TokenType::Number,
            lexeme: "42".to_string(),
            literal: "42".to_string(),
            line: 1,
        })), Token {
            token_type: TokenType::Plus,
            lexeme: "+".to_string(),
            literal: "".to_string(),
            line: 1,
        }, Box::new(Expr::Literal(Token {
            token_type: TokenType::Number,
            lexeme: "42".to_string(),
            literal: "42".to_string(),
            line: 1,
        }))).to_string(), "(42 + 42)");

        assert_eq!(Expr::Grouping(Box::new(Expr::Literal(Token {
            token_type: TokenType::Number,
            lexeme: "42".to_string(),
            literal: "42".to_string(),
            line: 1,
        }))).to_string(), "(42)");
    }
}
