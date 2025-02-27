mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;

pub use scanner::Scanner;

/*
struct Parser<'a>{
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Parser<'a> {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.match_next(TokenType::LeftParen) {
            return Box::new(Expr::
        }
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return expr;
        }
    }
}
*/
