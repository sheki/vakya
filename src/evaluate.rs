use crate::expr::{Expr, Value};
use crate::parser_error::Error;
use crate::token_type::TokenType;

fn is_truthy(value: Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Boolean(false) => false,
        _ => true,
    }
}

fn numeric(v: &Value) -> Result<f64, Error> {
    match v {
        Value::Number(num) => Ok(*num),
        _ => return Err(Error::EvalError("Operand must be a number".to_string())),
    }
}

fn is_equal(right: &Value, left: &Value) -> Result<bool, Error> {
    match (right, left) {
        (Value::Number(num1), Value::Number(num2)) => Ok(num1 == num2),
        (Value::String(str1), Value::String(str2)) => Ok(str1 == str2),
        (Value::Boolean(bool1), Value::Boolean(bool2)) => Ok(bool1 == bool2),
        (Value::Nil, Value::Nil) => Ok(true),
        _ => {
            return Err(Error::EvalError(
                "Operands must be of same type".to_string(),
            ))
        }
    }
}

pub fn evaluate(expr: Box<Expr>) -> Result<Value, Error> {
    match *expr {
        Expr::Literal(value) => Ok(value),
        Expr::Grouping(expr) => evaluate(expr),
        Expr::Unary(token, expr_right) => {
            let right = evaluate(expr_right)?;
            match token.token_type {
                TokenType::Minus => match right {
                    Value::Number(num) => Ok(Value::Number(-num)),
                    _ => return Err(Error::EvalError("Operand must be a number".to_string())),
                },
                TokenType::Bang => Ok(Value::Boolean(!is_truthy(right))),
                _ => return Err(Error::EvalError("Unknown operator".to_string())),
            }
        }
        Expr::Binary(expr_left, token, expr_right) => {
            let left = evaluate(expr_left)?;
            let right = evaluate(expr_right)?;
            match token.token_type {
                TokenType::Plus => {
                    return Ok(Value::Number(numeric(&left)? + numeric(&right)?));
                }
                TokenType::Minus => {
                    return Ok(Value::Number(numeric(&left)? - numeric(&right)?));
                }
                TokenType::Star => {
                    return Ok(Value::Number(numeric(&left)? * numeric(&right)?));
                }
                TokenType::Slash => {
                    return Ok(Value::Number(numeric(&left)? / numeric(&right)?));
                }
                TokenType::Greater => {
                    return Ok(Value::Boolean(numeric(&left)? > numeric(&right)?));
                }
                TokenType::GreaterEqual => {
                    return Ok(Value::Boolean(numeric(&left)? >= numeric(&right)?));
                }
                TokenType::Less => {
                    return Ok(Value::Boolean(numeric(&left)? < numeric(&right)?));
                }
                TokenType::LessEqual => {
                    return Ok(Value::Boolean(numeric(&left)? <= numeric(&right)?));
                }
                TokenType::BangEqual => {
                    return Ok(Value::Boolean(!is_equal(&left, &right)?));
                }
                TokenType::EqualEqual => {
                    return Ok(Value::Boolean(is_equal(&left, &right)?));
                }
                _ => return Err(Error::EvalError("Unknown operator".to_string())),
            }
        }
    }
}
