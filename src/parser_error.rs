use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("parser error {0}")]
    ParserError(String),
    #[error("eval error error {0}")]
    EvalError(String),
}
