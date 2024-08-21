use std::fmt::Display;

use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, Clone, Copy)]
pub enum TokenKind {
    Number,
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LParen,
    RParen,
    Function,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    FuncParams(String, u8), // function_name, num_of_params
    StrValue(String),
    NumValue(f64),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
}

impl Token {
    pub fn new(kind: TokenKind, value: Option<TokenValue>) -> Token {
        if value.is_none() {
            return Self {
                kind,
                value: TokenValue::None,
            };
        }
        Self {
            kind,
            value: value.unwrap(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value == TokenValue::None {
            write!(f, "{}", self.kind)
        } else {
            write!(f, "{}:{:?}", self.kind, self.value)
        }
    }
}
