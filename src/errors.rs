use std::fmt::Display;

use colored::Colorize;

#[derive(Debug, Clone)]
pub enum ErrorReason {
    Error(String),
}

impl Display for ErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorReason::Error(reason) => write!(f, "{}", reason.red()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParserError {
    #[allow(dead_code)]
    pub error: ErrorReason,
    pub description: String,
}

impl ParserError {
    pub fn new(error: ErrorReason) -> Self {
        Self {
            description: error.to_string(),
            error,
        }
    }
}
