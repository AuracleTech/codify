use jayce::duos;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Lexemes {
    Whitespace,
    CommentLine,
    CommentBlock,
    Newline,

    Identifier,
    String,
    Number,

    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Assign,

    Power,

    LeftParenthesis,
    RightParenthesis,

    Comma,
}

lazy_static::lazy_static! {
pub static ref LEXEMES_CODIFY: Vec<(Lexemes, Regex)> =  duos!(
    Lexemes::Whitespace, r"^[^\S\n]+",
    Lexemes::CommentLine, r"^//(.*)",
    Lexemes::CommentBlock, r"^/\*(.|\n)*?\*/",
    Lexemes::Newline, r"^\n",

    Lexemes::Identifier, r"^[a-zA-Z_][a-zA-Z0-9_]*",
    Lexemes::String, r#"^"[^"]*""#,
    Lexemes::Number, r"^\d+",

    Lexemes::Add, r"^\+",
    Lexemes::Subtract, r"^-",
    Lexemes::Multiply, r"^\*",
    Lexemes::Divide, r"^/",
    Lexemes::Modulo, r"^%",
    Lexemes::Assign, r"^=",

    Lexemes::Power, r"^\^",

    Lexemes::LeftParenthesis, r"^\(",
    Lexemes::RightParenthesis, r"^\)",

    Lexemes::Comma, r"^,"
);
}
