fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    Ok(())
}

/*
mod lib;
use crate::lib::Parser;
use lib::rules::RuleSet;

const RULES: &str = include_str!("../code/main.rules");
// const SOURCE: &str = include_str!("../code/main.code");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ruleset = RuleSet::from(RULES)?;
    println!("rules: {:#?}", ruleset);
    let mut parser = Parser::new(ruleset);

    // parser.parse_programn()?;

    // println!("AST: {:#?}", parser.program);

    Ok(())
}

*/
