mod parser;
use parser::{kinds::LEXEMES_CODIFY, Parser};

const SOURCE: &str = include_str!("../code/main.code");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = Parser::new(SOURCE, &LEXEMES_CODIFY);

    parser.parse_programn()?;

    println!("AST: {:#?}", parser.program);

    Ok(())
}
