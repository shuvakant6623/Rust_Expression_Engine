use std::fmt;

#[derive(Debug)]
pub enum ExpressionError {
    Lexer { message: String, position: usize },
    Parser { message: String },
    Evaluation { message: String },
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lexer { message, position } =>
                write!(f, "Lexer error at {}: {}", position, message),
            Self::Parser { message } =>
                write!(f, "Parser error: {}", message),
            Self::Evaluation { message } =>
                write!(f, "Evaluation error: {}", message),
        }
    }
}

impl std::error::Error for ExpressionError {}
