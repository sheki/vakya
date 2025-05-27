use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug)]
pub enum Stmt<'a> {
    //
    ExprStmt(Box<Expr<'a>>),
    PrintStmt(Box<Expr<'a>>),
    VarStmt(&'a Token, Option<Box<Expr<'a>>>),
}
