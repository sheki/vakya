use crate::env::Env;
use crate::evaluate::evaluate;
use crate::expr::Value;
use crate::parser_error::Error;
use crate::stmt::Stmt;

pub struct Interpreter {
    env: Env,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: Env::new() }
    }

    pub fn evaluate_stmt(&mut self, stmt: Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::ExprStmt(expr) => {
                // Evaluate the expression but don't print the result
                evaluate(*expr, &self.env)?;
                Ok(())
            }
            Stmt::PrintStmt(expr) => {
                // Print the result of evaluating the expression
                println!("{:?}", evaluate(*expr, &self.env)?);
                Ok(())
            }
            Stmt::VarStmt(name_token, initializer) => {
                if let Some(initializer) = initializer {
                    let value = evaluate(*initializer, &self.env)?;
                    self.env.define(name_token.lexeme.clone(), value);
                } else {
                    self.env.define(name_token.lexeme.clone(), Value::Nil);
                }
                Ok(())
            }
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            let eval_result = self.evaluate_stmt(stmt);
            if let Err(error) = eval_result {
                println!("error: {:?}", error);
            }
        }
    }
}
