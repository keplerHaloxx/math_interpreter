use std::iter::Peekable;

use crate::{
    errors::ErrorReason,
    token::{Token, TokenKind, TokenValue},
};

const DIGITS: &str = ".0123456789";
#[allow(unused)]
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

pub struct Lexer<I: Iterator<Item = char> + Clone> {
    // source: String,
    iter: Peekable<I>,
}

impl<I: Iterator<Item = char> + Clone> Lexer<I> {
    pub fn new(source: I) -> Self {
        Self {
            // source,
            iter: source.peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ErrorReason> {
        let mut tokens: Vec<Token> = Vec::new();

        // Best solution i could find to making sure it doesn't skip
        while let Some(&current) = self.iter.peek() {
            if current.is_ascii_whitespace() {
                self.iter.next();
                continue;
            }

            if DIGITS.contains(current) {
                tokens.push(self.generate_number()?);
            } else {
                let token = match current {
                    '+' => Token::new(TokenKind::Plus, None),
                    '-' => Token::new(TokenKind::Minus, None),
                    '*' => Token::new(TokenKind::Multiply, None),
                    '/' => Token::new(TokenKind::Divide, None),
                    '^' => Token::new(TokenKind::Power, None),
                    '(' => Token::new(TokenKind::LParen, None),
                    ')' => Token::new(TokenKind::RParen, None),
                    // 'a'..='z' => {}
                    _ => Token::new(
                        TokenKind::Unknown,
                        Some(TokenValue::StrValue(current.to_string())),
                    ),
                };
                tokens.push(token);
                self.iter.next();
            }
        }

        Ok(tokens)
    }

    #[allow(unused)]
    fn read_function(&mut self) -> Token {
        // TODO: read the parameters of function
        // bruh i cant figure this out rn i do later
        let mut name = String::new();

        while let Some(&current) = self.iter.peek() {
            if LETTERS.contains(current) {
                name.push(current);
            }

            if current == '(' {
                self.iter.next(); // Consume '('
            }
            self.iter.next();
        }
        Token::new(TokenKind::Function, Some(TokenValue::FuncParams(name, 0))) // change params later
    }

    fn generate_number(&mut self) -> Result<Token, ErrorReason>
    where
        I: Iterator<Item = char>,
    {
        let mut decimal_point_counter = 0;
        let mut number = String::new();

        while let Some(&current) = self.iter.peek() {
            if current == '.' {
                decimal_point_counter += 1;
                if decimal_point_counter > 1 {
                    return Err(ErrorReason::Error("Too many decimal points".into()));
                }
            }
            number.push(current);
            self.iter.next();

            // Peek the next character and check if it's valid for a number
            if let Some(&next_char) = self.iter.peek() {
                println!("still ehre");
                if !DIGITS.contains(next_char) {
                    if number.trim() == "." {
                        return Err(ErrorReason::Error("Invalid sequence".into()));
                    }
                    break;
                }
            } else {
                return Err(ErrorReason::Error("Invalid sequence".into()));
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
