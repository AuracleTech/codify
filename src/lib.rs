// pub mod kinds;
// use self::kinds::Lexemes;
// use errors::ParserErrors;
// use jayce::{duos, Token, Tokenizer};
// use regex::Regex;
pub mod rules;
pub use rules::RuleSet;
// use rules::{Component, Rule, RuleSet};

pub struct Parser {
    pub ruleset: RuleSet,
    // tokenizer: Tokenizer<'a, String>,
    // pub program: Vec<ASTNode>,
    // pub option_token: Option<Token<'a, Lexemes>>,
}

// #[derive(Debug)]
// pub enum ASTNode {
//     Program(Vec<ASTNode>),
//     Statement(Box<ASTNode>),
//     Expression(Box<ASTNode>),
//     Assign {
//         name: String,
//         expression: Box<ASTNode>,
//     },
//     Invoke {
//         name: String,
//         argument: Box<ASTNode>,
//     },
//     Parenthesis(Box<ASTNode>),
//     Addition {
//         left: Box<ASTNode>,
//         right: Box<ASTNode>,
//     },
//     Subtraction {
//         left: Box<ASTNode>,
//         right: Box<ASTNode>,
//     },
//     Multiplication {
//         left: Box<ASTNode>,
//         right: Box<ASTNode>,
//     },
//     Division {
//         left: Box<ASTNode>,
//         right: Box<ASTNode>,
//     },
// }

impl Parser {
    pub fn new(ruleset: RuleSet) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            // tokenizer: Tokenizer::new(source, &DUOS),
            ruleset,
            // program: vec![],
            // option_token: None,
        })
    }

    // pub fn next(&mut self) -> Result<(), ParserErrors<'a, Lexemes>> {
    //     match self.tokenizer.next() {
    //         Ok(option_token) => self.option_token = option_token,
    //         Err(err) => return Err(ParserErrors::NoTokenMatched(err)),
    //     };

    //     Ok(())
    // }

    // pub fn get_token(&self, caller: &str) -> Result<Token<'a, Lexemes>, ParserErrors<'a, Lexemes>> {
    //     match self.option_token {
    //         Some(ref token) => Ok(token.clone()),
    //         None => Err(ParserErrors::UnexpectedEndOfFile(caller)),
    //     }
    // }

    // pub fn parse_programn(&mut self) -> Result<(), ParserErrors<'a, Lexemes>> {
    //     self.next()?;

    //     while self.option_token.is_some() {
    //         let ast = self.parse_statements()?;
    //         self.program.push(ast);
    //     }

    //     Ok(())
    // }

    // fn parse_statements(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
    //     let token = self.get_token("parse_statements 0")?;

    //     match token.kind {
    //         Lexemes::Identifier => {
    //             let name = token.value.to_string();

    //             let next = self.get_token("parse_statements 1")?;
    //             self.expect_lexeme("=", Lexemes::Assign)?;
    //             let expr = self.parse_expression()?;
    //             Ok(ASTNode::Assign {
    //                 name,
    //                 expression: Box::new(expr),
    //             });

    //             match ASSIGN_OR_INVOKE_OR? {
    //                 assign {},
    //                 invoke {},
    //                 _ => Err(ParserErrors::UnexpectedToken(token.clone()))?,
    //             }

    //         }
    //         _ => {
    //             return Err(ParserErrors::UnexpectedToken(token.clone()));
    //         }
    //     }

    //     match self.option_token.take() {
    //         Some(token) => {
    //             ast = match token.kind {
    //                 Lexemes::Identifier if id == "print" => {
    //                     self.next()?;
    //                     let expr = self.parse_expression()?;
    //                     ASTNode::Function {
    //                         name: String::from("print"),
    //                         argument: Box::new(expr),
    //                     }
    //                 }
    //                 Lexemes::Identifier => {
    //                     let id = token.lexeme;
    //                     self.next()?;
    //                     self.expect_lexeme("=", Lexemes::Assignment)?;
    //                     let expr = self.parse_expression()?;
    //                     ASTNode::Assignment {
    //                         id: String::from(id),
    //                         expression: Box::new(expr),
    //                     }
    //                 }
    //                 _ => {
    //                     return Err(ParserErrors::UnexpectedToken(token.clone()));
    //                 }
    //             };
    //         }
    //         None => {
    //             return Err(ParserErrors::UnexpectedEndOfFile("parse_statements"));
    //         }
    //     }

    //     Ok(ast)
    // }

    // // fn parse_statements(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {

    // //     parse_assignment() || parse_

    // //     match self.current.take() {
    // //         Some(token) => match token.kind {
    // //             Lexemes::Identifier => {
    // //                 let identifier = token.value.to_string();
    // //                 self.advance()?;
    // //                 match self.current.take() {
    // //                     Some(token) => match token.kind {
    // //                         Lexemes::Assign => Ok(ASTNode::Assignment {
    // //                             identifier,
    // //                             expression: Box::new(self.parse_expression()?), // TODO maytbe parse_assign
    // //                         }),
    // //                         Lexemes::Identifier | Lexemes::Number | Lexemes::String => {
    // //                             Ok(ASTNode::FunctionCall {
    // //                                 name: identifier,
    // //                                 arguments: self.parse_arguments()?,
    // //                             })
    // //                         }
    // //                         _ => Err(ParserErrors::UnimplementedToken(
    // //                             token,
    // //                             "parse_statements 1",
    // //                         ))?,
    // //                     },
    // //                     None => Err(ParserErrors::UnexpectedEndOfFile("parse_statements 1"))?,
    // //                 }
    // //             }
    // //             // Lexemes::Print => self.parse_print(),
    // //             // Lexemes::If => self.parse_if(),
    // //             // Lexemes::While => self.parse_while(),
    // //             // Lexemes::Function => self.parse_function(),
    // //             _ => Err(ParserErrors::UnimplementedToken(
    // //                 token,
    // //                 "parse_statements 2",
    // //             ))?,
    // //         },
    // //         None => Err(ParserErrors::UnexpectedEndOfFile("parse_statements 2"))?,
    // //     }
    // // }

    // fn parse_assign(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
    //     let identifier = match self.option_token.take() {
    //         Some(token) => match token.kind {
    //             Lexemes::Identifier => token.value.to_string(),
    //             _ => Err(ParserErrors::UnimplementedToken(token, "parse_assign 1"))?,
    //         },
    //         None => Err(ParserErrors::UnexpectedEndOfFile("parse_assign 1"))?,
    //     };

    //     self.next()?;

    //     match self.option_token.take() {
    //         Some(token) => match token.kind {
    //             Lexemes::Assign => (),
    //             _ => Err(ParserErrors::UnimplementedToken(token, "parse_assign 2"))?,
    //         },
    //         None => Err(ParserErrors::UnexpectedEndOfFile("parse_assign 2"))?,
    //     }

    //     self.next()?;

    //     let value = self.parse_expression()?;

    //     Ok(ASTNode::Assignment {
    //         identifier,
    //         expression: Box::new(value),
    //     })
    // }

    // fn parse_expression(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
    //     let mut left_operand = self.parse_term()?;

    //     while self.option_token.is_some() {
    //         match self.option_token.take() {
    //             Some(token) => match token.kind {
    //                 Lexemes::Add | Lexemes::Subtract => {
    //                     let operator = token.value.to_string();
    //                     self.next()?;
    //                     let right_operand = self.parse_term()?;
    //                     left_operand = ASTNode::BinaryOperation {
    //                         operator,
    //                         left_operand: Box::new(left_operand),
    //                         right_operand: Box::new(right_operand),
    //                     };
    //                 }
    //                 _ => break,
    //             },
    //             None => break,
    //         }
    //     }

    //     Ok(left_operand)
    // }

    // fn parse_term(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
    //     let mut left_operand = self.parse_factor()?;

    //     while self.option_token.is_some() {
    //         match self.option_token.take() {
    //             Some(token) => match token.kind {
    //                 Lexemes::Multiply | Lexemes::Divide | Lexemes::Modulo => {
    //                     let operator = token.value.to_string();
    //                     self.next()?;
    //                     let right_operand = self.parse_factor()?;
    //                     left_operand = ASTNode::BinaryOperation {
    //                         operator,
    //                         left_operand: Box::new(left_operand),
    //                         right_operand: Box::new(right_operand),
    //                     };
    //                 }
    //                 _ => break,
    //             },
    //             None => break,
    //         }
    //     }

    //     Ok(left_operand)
    // }

    // fn parse_factor(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
    //     let mut left_operand = self.parse_power()?;

    //     while self.option_token.is_some() {
    //         match self.option_token.take() {
    //             Some(token) => match token.kind {
    //                 Lexemes::Power => {
    //                     let operator = token.value.to_string();
    //                     self.next()?;
    //                     let right_operand = self.parse_power()?;
    //                     left_operand = ASTNode::BinaryOperation {
    //                         operator,
    //                         left_operand: Box::new(left_operand),
    //                         right_operand: Box::new(right_operand),
    //                     };
    //                 }
    //                 _ => break,
    //             },
    //             None => break,
    //         }
    //     }

    //     Ok(left_operand)
    // }

    // fn parse_power(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
    //     let mut left_operand = self.parse_unary()?;

    //     while self.option_token.is_some() {
    //         match self.option_token.take() {
    //             Some(token) => match token.kind {
    //                 Lexemes::Power => {
    //                     let operator = token.value.to_string();
    //                     self.next()?;
    //                     let right_operand = self.parse_unary()?;
    //                     left_operand = ASTNode::BinaryOperation {
    //                         operator,
    //                         left_operand: Box::new(left_operand),
    //                         right_operand: Box::new(right_operand),
    //                     };
    //                 }
    //                 _ => break,
    //             },
    //             None => break,
    //         }
    //     }

    //     Ok(left_operand)
    // }

    // fn parse_unary(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
    //     match self.option_token.take() {
    //         Some(token) => match token.kind {
    //             Lexemes::Add | Lexemes::Subtract => {
    //                 let operator = token.value.to_string();
    //                 self.next()?;
    //                 let operand = self.parse_unary()?;
    //                 Ok(ASTNode::BinaryOperation {
    //                     operator,
    //                     left_operand: Box::new(ASTNode::Number(0.0)),
    //                     right_operand: Box::new(operand),
    //                 })
    //             }
    //             _ => self.parse_primary(),
    //         },
    //         None => Err(ParserErrors::UnexpectedEndOfFile("parse_unary"))?,
    //     }
    // }

    // fn parse_primary(&mut self) -> Result<ASTNode, ParserErrors<'a, Lexemes>> {
    //     match self.option_token.take() {
    //         Some(token) => match token.kind {
    //             Lexemes::Number => Ok(ASTNode::Number(token.value.parse::<f32>().unwrap())),
    //             Lexemes::String => Ok(ASTNode::String(token.value.to_string())),
    //             Lexemes::Identifier => {
    //                 let identifier = token.value.to_string();
    //                 self.next()?;
    //                 match self.option_token.take() {
    //                     Some(token) => match token.kind {
    //                         Lexemes::LeftParenthesis => Ok(ASTNode::FunctionCall {
    //                             name: identifier,
    //                             arguments: self.parse_arguments()?,
    //                         }),
    //                         _ => Ok(ASTNode::Identifier(identifier)),
    //                     },
    //                     None => Ok(ASTNode::Identifier(identifier)),
    //                 }
    //             }
    //             Lexemes::LeftParenthesis => {
    //                 let expression = self.parse_expression()?;
    //                 match self.option_token.take() {
    //                     Some(token) => match token.kind {
    //                         Lexemes::RightParenthesis => Ok(expression),
    //                         _ => Err(ParserErrors::UnimplementedToken(token, "parse_primary"))?,
    //                     },
    //                     None => Err(ParserErrors::UnexpectedEndOfFile("parse_primary"))?,
    //                 }
    //             }
    //             _ => Err(ParserErrors::UnimplementedToken(token, "parse_primary"))?,
    //         },
    //         None => Err(ParserErrors::UnexpectedEndOfFile("parse_primary"))?,
    //     }
    // }

    // fn parse_arguments(&mut self) -> Result<Vec<ASTNode>, ParserErrors<'a, Lexemes>> {
    //     self.next()?;

    //     let mut arguments = Vec::new();

    //     while self.option_token.is_some() {
    //         match self.option_token.take() {
    //             Some(token) => match token.kind {
    //                 Lexemes::RightParenthesis => break,
    //                 Lexemes::Comma => (),
    //                 _ => {
    //                     let argument = self.parse_expression()?;
    //                     arguments.push(argument);
    //                 }
    //             },
    //             None => Err(ParserErrors::UnexpectedEndOfFile("parse_arguments"))?,
    //         }

    //         self.next()?;
    //     }

    //     Ok(arguments)
    // }
}
