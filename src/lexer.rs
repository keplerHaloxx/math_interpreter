use std::iter::Peekable;

use crate::{
    errors::ErrorReason,
    token::{Token, TokenKind, TokenValue},
};

const DIGITS: &str = ".0123456789";

pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, ErrorReason> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = self
            .source
            .chars()
            .filter(|c| !c.is_whitespace())
            .peekable();

        while let Some(&current) = chars.peek() {
            // Best solution i could find to making sure it doesn't skip
            if DIGITS.contains(current) {
                tokens.push(self.generate_number(&mut chars)?);
            } else {
                let token = match current {
                    '+' => Token::new(TokenKind::Plus, None),
                    '-' => Token::new(TokenKind::Minus, None),
                    '*' => Token::new(TokenKind::Multiply, None),
                    '/' => Token::new(TokenKind::Divide, None),
                    '^' => Token::new(TokenKind::Power, None),
                    '(' => Token::new(TokenKind::LParen, None),
                    ')' => Token::new(TokenKind::RParen, None),
                    _ => Token::new(
                        TokenKind::Unknown,
                        Some(TokenValue::StrValue(current.to_string())),
                    ),
                };
                tokens.push(token);
                chars.next();
            }
        }

        Ok(tokens)
    }

    fn generate_number<I>(&self, chars: &mut Peekable<I>) -> Result<Token, ErrorReason>
    where
        I: Iterator<Item = char>,
    {
        let mut decimal_point_counter = 0;
        let mut number = String::new();

        while let Some(&current) = chars.peek() {
            if current == '.' {
                decimal_point_counter += 1;
                if decimal_point_counter > 1 {
                    return Err(ErrorReason::Error("Too many decimal points".into()));
                }
            }
            number.push(current);
            chars.next();

            // Peek the next character and check if it's valid for a number
            if let Some(&next_char) = chars.peek() {
                if !DIGITS.contains(next_char) {
                    if number.trim() == "." {
                        return Err(ErrorReason::Error("Random decimal place found ".into()));
                    }
                    break;
                }
            }
        }

        Ok(Token::new(
            TokenKind::Number,
            Some(TokenValue::NumValue(number.parse::<f64>().unwrap_or_else(
                |_| panic!("Error parsing number '{number}'"),
            ))),
        ))
    }
}
