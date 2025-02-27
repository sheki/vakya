use crate::token::Token;

#[derive(Debug)]
pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    Literal(Value),
    Unary(&'a Token, Box<Expr<'a>>),
}
#[derive(Debug)]
pub enum Value {
    Number(String),
    String(String),
    Boolean(bool),
    Nil,
}
