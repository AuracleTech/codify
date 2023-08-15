use jayce::Token;
use std::fmt;

#[derive(Debug)]
pub enum ParserErrors<'a, T> {
    // Token errors
    NoTokenMatched(Box<dyn std::error::Error>),
    UnexpectedEndOfFile(&'a str),
    UnexpectedToken(Token<'a, T>),
    UnimplementedToken(Token<'a, T>, &'a str),

    // Parser errors
    InvalidNumber(Token<'a, T>),
    // Lexical errors

    // Syntactical errors

    // Semantic errors

    // Runtime errors
}

impl<'a, T> std::error::Error for ParserErrors<'a, T> where T: fmt::Debug {}

impl<'a, T> fmt::Display for ParserErrors<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrors::NoTokenMatched(err) => write!(f, "{}", err),
            ParserErrors::UnexpectedEndOfFile(caller) => {
                write!(f, "Unexpected end of file from {}", caller)
            }
            ParserErrors::UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
            ParserErrors::UnimplementedToken(token, caller) => {
                write!(f, "Unimplemented token: {:?} from {}", token, caller)
            }

            ParserErrors::InvalidNumber(token) => write!(f, "Invalid number: {:?}", token),
        }
    }
}
