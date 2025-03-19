use crate::token::Token;

#[derive(Debug)]
pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    Literal(Value),
    Unary(&'a Token, Box<Expr<'a>>),
}
#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_equals() {
        assert_eq!(Value::Number(42.0), Value::Number(42.0));
        assert_eq!(Value::Boolean(true), Value::Boolean(true));
        assert_ne!(Value::Number(42.0), Value::Number(43.0));
        assert_ne!(Value::Boolean(true), Value::Boolean(false));
    }
}
