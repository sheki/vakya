use crate::expr::Expr;

#[derive(Debug)]
pub enum Stmt<'a> {
    //
    ExprStmt(Box<Expr<'a>>),
    PrintStmt(Box<Expr<'a>>),
}
