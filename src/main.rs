mod errors;
mod lexer;
mod parser;
mod token;
mod traits;

use std::io::{stdin, stdout, Write};

use colored::Colorize;
use errors::{ErrorReason, ParserError};
use lexer::Lexer;
use parser::Parser;
use token::{Token, TokenKind, TokenValue};

/// Prompts user for input and returns trimmed result
fn prompt_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    input.trim().to_owned()
}

fn main() {
    println!("MATH INTERPRETER");
    println!("----------------");

    loop {
        let input = prompt_input("> ");

        let mut lexer = Lexer::new(input.chars());
        let tokens_result = lexer.tokenize();
        if let Err(err) = tokens_result {
            println!("{}", err);
            continue;
        }
        let tokens = tokens_result.unwrap();

        let token_errors = get_tokens_errors(tokens.clone());
        if !token_errors.is_empty() {
            for err in token_errors {
                println!(
                    "{}",
                    ParserError::new(ErrorReason::Error(format!("Invalid sequence: '{}'", err)))
                        .description
                );
            }
            println!();
        } else {
            // pretty_print_tokens(tokens.clone());
            let mut parser = Parser::new(tokens.into_iter());
            let result = parser.parse_expr();
            if result.is_err() {
                println!("{}", result.unwrap_err().description);
                continue;
            }
            println!(
                "{} {}",
                "RESULT:".bright_green(),
                result.unwrap().evaluate().to_string().bright_green()
            );
        }
    }
}

#[allow(dead_code)]
fn pretty_print_tokens(tokens: Vec<Token>) {
    let token_len = tokens.len();
    tokens.iter().enumerate().for_each(|(i, token)| {
        if i == token_len - 1 {
            println!("{token}");
        } else {
            print!("{}, ", token);
        }
    })
}

fn get_tokens_errors(tokens: Vec<Token>) -> Vec<String> {
    let mut error_str: Vec<String> = vec![];
    let mut current_sequence: String = String::new();
    let mut tokens_iter = tokens.iter().peekable();
    while let Some(token) = tokens_iter.next() {
        // If there's multiple correct tokens just skip that
        if token.kind != TokenKind::Unknown {
            continue;
        }
        if let TokenValue::StrValue(value) = &token.value {
            // Push illegal char into current sequence
            current_sequence.push(value.chars().next().unwrap());
        }

        // Check if next value is empty or not unknown
        // If true then push current sequence to end string
        let peek = tokens_iter.peek();
        if !current_sequence.is_empty()
            && (peek.is_none() || peek.unwrap().kind != TokenKind::Unknown)
        {
            // Append the current sequence
            error_str.push(current_sequence.clone());
            current_sequence.clear();
        }
    }
    error_str
}
