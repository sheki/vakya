use crate::env::Env;
use crate::expr::{Expr, Value};
use crate::parser_error::Error;
use crate::token_type::TokenType;

fn is_truthy(value: &Value) -> bool {
    !matches!(value, Value::Nil | Value::Boolean(false))
}

fn numeric(v: &Value) -> Result<f64, Error> {
    match v {
        Value::Number(num) => Ok(*num),
        _ => Err(Error::EvalError("Operand must be a number".to_string())),
    }
}

fn is_equal(right: &Value, left: &Value) -> Result<bool, Error> {
    match (right, left) {
        (Value::Number(num1), Value::Number(num2)) => Ok(num1 == num2),
        (Value::String(str1), Value::String(str2)) => Ok(str1 == str2),
        (Value::Boolean(bool1), Value::Boolean(bool2)) => Ok(bool1 == bool2),
        (Value::Nil, Value::Nil) => Ok(true),
        _ => Err(Error::EvalError(
            "Operands must be of same type".to_string(),
        )),
    }
}

pub fn evaluate<'a>(expr: Expr<'a>, env: &'a Env) -> Result<Value, Error> {
    match expr {
        Expr::Literal(value) => Ok(value),
        Expr::Grouping(expr) => evaluate(*expr, env),
        Expr::Unary(token, expr_right) => {
            let right = evaluate(*expr_right, env)?;
            match token.token_type {
                TokenType::Minus => match right {
                    Value::Number(num) => Ok(Value::Number(-num)),
                    _ => Err(Error::EvalError("Operand must be a number".to_string())),
                },
                TokenType::Bang => Ok(Value::Boolean(!is_truthy(&right))),
                _ => Err(Error::EvalError("Unknown operator".to_string())),
            }
        }
        Expr::Binary(expr_left, token, expr_right) => {
            let left = evaluate(*expr_left, env)?;
            let right = evaluate(*expr_right, env)?;
            match token.token_type {
                TokenType::Plus => Ok(Value::Number(numeric(&left)? + numeric(&right)?)),
                TokenType::Minus => Ok(Value::Number(numeric(&left)? - numeric(&right)?)),
                TokenType::Star => Ok(Value::Number(numeric(&left)? * numeric(&right)?)),
                TokenType::Slash => Ok(Value::Number(numeric(&left)? / numeric(&right)?)),
                TokenType::Greater => Ok(Value::Boolean(numeric(&left)? > numeric(&right)?)),
                TokenType::GreaterEqual => Ok(Value::Boolean(numeric(&left)? >= numeric(&right)?)),
                TokenType::Less => Ok(Value::Boolean(numeric(&left)? < numeric(&right)?)),
                TokenType::LessEqual => Ok(Value::Boolean(numeric(&left)? <= numeric(&right)?)),
                TokenType::BangEqual => Ok(Value::Boolean(!is_equal(&left, &right)?)),
                TokenType::EqualEqual => Ok(Value::Boolean(is_equal(&left, &right)?)),
                _ => Err(Error::EvalError("Unknown operator".to_string())),
            }
        }
        Expr::Variable(name_token_ref) => {
            let name = &name_token_ref.lexeme;
            if let Some(value_ref) = env.get(name) {
                // TODO fix clone
                Ok(value_ref.clone())
            } else {
                Err(Error::EvalError(format!("Undefined variable '{}'", name)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_evaluate_literal() {
        let expr = Expr::Literal(Value::Number(42.0));
        let result = evaluate(expr, &Env::new()).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_evaluate_unary_minus() {
        let token = Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: "".to_string(),
            line: 1,
        };
        let expr = Expr::Unary(&token, Box::new(Expr::Literal(Value::Number(42.0))));
        let result = evaluate(expr, &Env::new()).unwrap();
        assert_eq!(result, Value::Number(-42.0));
    }
}
