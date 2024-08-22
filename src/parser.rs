// - i needed some help making the AST. i dont think i could've made ts by myself 😭

use std::iter::Peekable;

use crate::{
    errors::{ErrorReason, ParserError},
    token::{Token, TokenKind, TokenValue},
    traits::Round,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ASTNode {
    Number(f64),
    BinaryOp(Box<ASTNode>, TokenKind, Box<ASTNode>),
}

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl ASTNode {
    pub fn evaluate(&self) -> f64 {
        match self {
            ASTNode::Number(val) => *val,
            ASTNode::BinaryOp(left_node, op, right_node) => {
                let left = left_node.evaluate();
                let right = right_node.evaluate();
                match op {
                    TokenKind::Plus => left + right,
                    TokenKind::Minus => left - right,
                    TokenKind::Multiply => left * right,
                    TokenKind::Divide => left / right,
                    TokenKind::Power => left.powf(right),
                    _ => panic!("wrong operation i cbf making proper errors for this"),
                }
                .round_to(5)
            }
        }
    }
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub fn parse_expr(&mut self) -> Result<ASTNode, ParserError> {
        let mut node = self.parse_term()?;

        while matches!(
            self.peek().map(|t| t.kind),
            Some(TokenKind::Plus) | Some(TokenKind::Minus) | Some(TokenKind::Power)
        ) {
            let operator = self.advance().unwrap().kind;
            let right = self.parse_term()?;
            node = ASTNode::BinaryOp(Box::new(node), operator, Box::new(right));
        }

        Ok(node)
    }

    fn parse_term(&mut self) -> Result<ASTNode, ParserError> {
        let mut node = self.parse_factor()?;

        while matches!(
            self.peek().map(|t| t.kind),
            Some(TokenKind::Multiply) | Some(TokenKind::Divide)
        ) {
            let operator = self.advance().unwrap().kind;
            let right = self.parse_factor()?;
            node = ASTNode::BinaryOp(Box::new(node), operator, Box::new(right));
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, ParserError> {
        match self.peek().map(|t| t.kind) {
            Some(TokenKind::Number) => {
                let value = if let TokenValue::NumValue(val) = self.advance().unwrap().value {
                    val
                } else {
                    return Err(ParserError::new(ErrorReason::Error(
                        "Expected a number".to_string(),
                    )));
                };
                Ok(ASTNode::Number(value))
            }
            Some(TokenKind::LParen) => {
                self.advance(); // Consume '('
                let node = self.parse_expr()?;
                if self.peek().map(|t| t.kind) != Some(TokenKind::RParen) {
                    return Err(ParserError::new(ErrorReason::Error(
                        "Expected ')'".into(),
                    )));
                }
                self.advance(); // Consume ')'
                Ok(node)
            }
            _ => Err(ParserError::new(ErrorReason::Error(format!(
                "Unexpected token: {:?}",
                if self.peek().is_some() {self.peek().unwrap().kind} else {TokenKind::Unknown}
            )))),
        }
    }
}
