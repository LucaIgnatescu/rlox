use thiserror::Error;

use crate::scanner::Token;

#[derive(Error, Debug, Default)]
#[error("line {line}, \"{lexeme}\": {message}")]
pub struct GenericError {
    line: u32,
    lexeme: String,
    message: String,
}

impl GenericError {
    pub fn new(t: &Token, message: &str) -> Self {
        Self {
            line: t.line,
            lexeme: t.lexeme.clone(),
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("Parse error: {0}")]
    ParseError(GenericError),

    #[error("Runtime error: {0}")]
    RuntimeError(GenericError),
}
