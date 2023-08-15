mod errors;
pub mod kinds;
use self::kinds::Lexemes;
use errors::ParserErrors;
use jayce::{Token, Tokenizer};
use regex::Regex;
mod symbol_table;
use symbol_table::SymbolTable;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a, Lexemes>,
    pub program: Vec<ASTNode>,
    pub current: Option<Token<'a, Lexemes>>,
    next: Option<Token<'a, Lexemes>>,
    symbol_table: SymbolTable,
}

#[derive(Debug)]
pub enum ASTNode {
    // Atomic nodes
    Identifier(String),
    String(String),
    Number(f32),

    // Compound nodes
    FunctionCall {
        name: String,
        arguments: Vec<ASTNode>,
    },

    Assignment {
        identifier: String,
        expression: Box<ASTNode>,
    },

    BinaryOperation {
        operator: String,
        left_operand: Box<ASTNode>,
        right_operand: Box<ASTNode>,
    },
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, duos: &'static [(Lexemes, Regex)]) -> Self {
        Self {
            tokenizer: Tokenizer::new(source, duos),
            program: vec![],
            current: None,
            next: None,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn advance(&mut self) -> Result<(), ParserErrors<'a, Lexemes>> {
        self.current = self.next.take();
        self.next = match self.tokenizer.next() {
            Ok(some_token) => some_token,
            Err(err) => return Err(ParserErrors::NoTokenMatched(err)),
        };

        Ok(())
    }

    pub fn parse_programn(&mut self) -> Result<(), ParserErrors<'a, Lexemes>> {
        self.advance()?;
        self.advance()?;

        while self.current.is_some() {
            let ast = self.parse_statements()?;
            self.program.push(ast);
        }

        Ok(())
    }

    fn parse_statements(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
        match self.current.take() {
            Some(token) => match token.kind {
                Lexemes::Identifier => {
                    let identifier = token.value.to_string();
                    self.advance()?;
                    match self.current.take() {
                        Some(token) => match token.kind {
                            Lexemes::Assign => Ok(ASTNode::Assignment {
                                identifier,
                                expression: Box::new(self.parse_expression()?), // TODO maytbe parse_assign
                            }),
                            Lexemes::Identifier | Lexemes::Number | Lexemes::String => {
                                Ok(ASTNode::FunctionCall {
                                    name: identifier,
                                    arguments: self.parse_arguments()?,
                                })
                            }
                            _ => Err(ParserErrors::UnimplementedToken(
                                token,
                                "parse_statements 1",
                            ))?,
                        },
                        None => Err(ParserErrors::UnexpectedEndOfFile("parse_statements 1"))?,
                    }
                }
                // Lexemes::Print => self.parse_print(),
                // Lexemes::If => self.parse_if(),
                // Lexemes::While => self.parse_while(),
                // Lexemes::Function => self.parse_function(),
                _ => Err(ParserErrors::UnimplementedToken(
                    token,
                    "parse_statements 2",
                ))?,
            },
            None => Err(ParserErrors::UnexpectedEndOfFile("parse_statements 2"))?,
        }
    }

    fn parse_assign(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
        let identifier = match self.current.take() {
            Some(token) => match token.kind {
                Lexemes::Identifier => token.value.to_string(),
                _ => Err(ParserErrors::UnimplementedToken(token, "parse_assign 1"))?,
            },
            None => Err(ParserErrors::UnexpectedEndOfFile("parse_assign 1"))?,
        };

        self.advance()?;

        match self.current.take() {
            Some(token) => match token.kind {
                Lexemes::Assign => (),
                _ => Err(ParserErrors::UnimplementedToken(token, "parse_assign 2"))?,
            },
            None => Err(ParserErrors::UnexpectedEndOfFile("parse_assign 2"))?,
        }

        self.advance()?;

        let value = self.parse_expression()?;

        Ok(ASTNode::Assignment {
            identifier,
            expression: Box::new(value),
        })
    }

    fn parse_expression(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
        let mut left_operand = self.parse_term()?;

        while self.current.is_some() {
            match self.current.take() {
                Some(token) => match token.kind {
                    Lexemes::Add | Lexemes::Subtract => {
                        let operator = token.value.to_string();
                        self.advance()?;
                        let right_operand = self.parse_term()?;
                        left_operand = ASTNode::BinaryOperation {
                            operator,
                            left_operand: Box::new(left_operand),
                            right_operand: Box::new(right_operand),
                        };
                    }
                    _ => break,
                },
                None => break,
            }
        }

        Ok(left_operand)
    }

    fn parse_term(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
        let mut left_operand = self.parse_factor()?;

        while self.current.is_some() {
            match self.current.take() {
                Some(token) => match token.kind {
                    Lexemes::Multiply | Lexemes::Divide | Lexemes::Modulo => {
                        let operator = token.value.to_string();
                        self.advance()?;
                        let right_operand = self.parse_factor()?;
                        left_operand = ASTNode::BinaryOperation {
                            operator,
                            left_operand: Box::new(left_operand),
                            right_operand: Box::new(right_operand),
                        };
                    }
                    _ => break,
                },
                None => break,
            }
        }

        Ok(left_operand)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
        let mut left_operand = self.parse_power()?;

        while self.current.is_some() {
            match self.current.take() {
                Some(token) => match token.kind {
                    Lexemes::Power => {
                        let operator = token.value.to_string();
                        self.advance()?;
                        let right_operand = self.parse_power()?;
                        left_operand = ASTNode::BinaryOperation {
                            operator,
                            left_operand: Box::new(left_operand),
                            right_operand: Box::new(right_operand),
                        };
                    }
                    _ => break,
                },
                None => break,
            }
        }

        Ok(left_operand)
    }

    fn parse_power(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
        let mut left_operand = self.parse_unary()?;

        while self.current.is_some() {
            match self.current.take() {
                Some(token) => match token.kind {
                    Lexemes::Power => {
                        let operator = token.value.to_string();
                        self.advance()?;
                        let right_operand = self.parse_unary()?;
                        left_operand = ASTNode::BinaryOperation {
                            operator,
                            left_operand: Box::new(left_operand),
                            right_operand: Box::new(right_operand),
                        };
                    }
                    _ => break,
                },
                None => break,
            }
        }

        Ok(left_operand)
    }

    fn parse_unary(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
        match self.current.take() {
            Some(token) => match token.kind {
                Lexemes::Add | Lexemes::Subtract => {
                    let operator = token.value.to_string();
                    self.advance()?;
                    let operand = self.parse_unary()?;
                    Ok(ASTNode::BinaryOperation {
                        operator,
                        left_operand: Box::new(ASTNode::Number(0.0)),
                        right_operand: Box::new(operand),
                    })
                }
                _ => self.parse_primary(),
            },
            None => Err(ParserErrors::UnexpectedEndOfFile("parse_unary"))?,
        }
    }

    fn parse_primary(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
        match self.current.take() {
            Some(token) => match token.kind {
                Lexemes::Number => Ok(ASTNode::Number(token.value.parse::<f32>().unwrap())),
                Lexemes::String => Ok(ASTNode::String(token.value.to_string())),
                Lexemes::Identifier => {
                    let identifier = token.value.to_string();
                    self.advance()?;
                    match self.current.take() {
                        Some(token) => match token.kind {
                            Lexemes::LeftParenthesis => Ok(ASTNode::FunctionCall {
                                name: identifier,
                                arguments: self.parse_arguments()?,
                            }),
                            _ => Ok(ASTNode::Identifier(identifier)),
                        },
                        None => Ok(ASTNode::Identifier(identifier)),
                    }
                }
                Lexemes::LeftParenthesis => {
                    let expression = self.parse_expression()?;
                    match self.current.take() {
                        Some(token) => match token.kind {
                            Lexemes::RightParenthesis => Ok(expression),
                            _ => Err(ParserErrors::UnimplementedToken(token, "parse_primary"))?,
                        },
                        None => Err(ParserErrors::UnexpectedEndOfFile("parse_primary"))?,
                    }
                }
                _ => Err(ParserErrors::UnimplementedToken(token, "parse_primary"))?,
            },
            None => Err(ParserErrors::UnexpectedEndOfFile("parse_primary"))?,
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<ASTNode>, ParserErrors<'a, Lexemes>> {
        self.advance()?;

        let mut arguments = Vec::new();

        while self.current.is_some() {
            match self.current.take() {
                Some(token) => match token.kind {
                    Lexemes::RightParenthesis => break,
                    Lexemes::Comma => (),
                    _ => {
                        let argument = self.parse_expression()?;
                        arguments.push(argument);
                    }
                },
                None => Err(ParserErrors::UnexpectedEndOfFile("parse_arguments"))?,
            }

            self.advance()?;
        }

        Ok(arguments)
    }
}
