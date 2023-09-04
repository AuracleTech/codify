use std::collections::HashMap;

use jayce::{duos, Token, Tokenizer};
use regex::Regex;

#[derive(Debug)]
pub enum RuleErrors {
    MissingIdentifierColonLineCol(usize, usize),
    EnforcedRuleUndefined(String),
    DuplicateRule(String),
    CalleeUndefined(String),
    UnusedRules(Vec<String>),
    NoParsingForRuleComponent,
    OnlyProgramIsAllowedRecusion,
}

impl std::error::Error for RuleErrors {}

impl std::fmt::Display for RuleErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleErrors::MissingIdentifierColonLineCol(line, col) => {
                write!(
                    f,
                    "Rule definition is missing the identifier colon ':' at line {}, col {}",
                    line, col
                )
            }
            RuleErrors::EnforcedRuleUndefined(rule) => {
                write!(f, "Enforced rule '{}' is undefined", rule)
            }
            RuleErrors::DuplicateRule(rule) => write!(f, "Rule '{}' is duplicated", rule),
            RuleErrors::CalleeUndefined(rule) => {
                write!(f, "Rule '{}' is called but undefined", rule)
            }
            RuleErrors::UnusedRules(rules) => {
                write!(f, "The following rules are unused: {:?}", rules)
            }
            RuleErrors::NoParsingForRuleComponent => {
                write!(f, "No parsing for rule component")
            }
            RuleErrors::OnlyProgramIsAllowedRecusion => {
                write!(f, "Recursion is only allowed for the program rule")
            }
        }
    }
}

#[derive(Debug)]
pub struct RuleSet {
    rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    name: String,
    components: Vec<Component>,
}

#[derive(Debug)]
pub enum Component {
    Regex { regex: Regex },
    RuleCall { name: String },
    Hardcoded { symbol: String },
    Alternative,
}

#[derive(Debug)]
pub enum DuoKinds {
    WhiteSpace,
    CommentLine,
    CommentBlock,
    Newline,

    RegexBlock,
    Regex,
    IdentifierColon,
    RuleCall,
    Alternative,
    Hardcoded,
}

lazy_static::lazy_static! {
pub static ref DUOS: Vec<(DuoKinds, Regex)> =  duos!(
    DuoKinds::WhiteSpace, r"^[^\S\n]+",
    DuoKinds::CommentLine, r"^//(.*)",
    DuoKinds::CommentBlock, r"^/\*(.|\n)*?\*/",
    DuoKinds::Newline, r"^\n",

    DuoKinds::RegexBlock, r#"^r#\".*\"\#"#,
    DuoKinds::Regex, r#"^(?P<regex>r"(?:[^"\\]|\\.)*"(?:\s*\+\s*r"(?:[^"\\]|\\.)*")*)"#,
    DuoKinds::IdentifierColon, r"^[a-zA-Z_][a-zA-Z0-9_]*:",
    DuoKinds::RuleCall, r"^[a-zA-Z_][a-zA-Z0-9_]*",
    DuoKinds::Alternative, r"^⇌",
    DuoKinds::Hardcoded, r#"^[^\s]*"#

);
}

impl RuleSet {
    pub fn from(source: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut rules_tokenizer = Tokenizer::new(source, &DUOS);

        let mut ruleset = Self { rules: vec![] };

        while let Some(token) = rules_tokenizer.next()? {
            match token.kind {
                DuoKinds::IdentifierColon => {
                    let mut rule = Rule {
                        name: token.value[..token.value.len() - 1].to_string(),
                        components: vec![],
                    };

                    while let Some(token) = rules_tokenizer.next()? {
                        match token.kind {
                            DuoKinds::Newline => break,
                            DuoKinds::WhiteSpace
                            | DuoKinds::CommentLine
                            | DuoKinds::CommentBlock => continue,
                            _ => rule.components.push(Component::from(token)?),
                        }
                    }

                    ruleset.rules.push(rule);
                }
                DuoKinds::CommentBlock
                | DuoKinds::CommentLine
                | DuoKinds::Newline
                | DuoKinds::WhiteSpace => continue,
                _ => {
                    return Err(
                        RuleErrors::MissingIdentifierColonLineCol(token.pos.0, token.pos.1).into(),
                    )
                }
            }
        }

        ruleset.verify_enforced_rule()?;
        ruleset.verify_duplicates()?;
        ruleset.verify_called_rules()?;
        ruleset.verify_unused_rules()?;
        ruleset.verify_recursion()?;

        Ok(ruleset)
    }

    fn verify_enforced_rule(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.rules.iter().any(|rule| rule.name == "program") {
            Ok(())
        } else {
            Err(RuleErrors::EnforcedRuleUndefined("program".to_string()).into())
        }
    }

    fn verify_duplicates(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut ruleset_rules = HashMap::new();

        for rule in &self.rules {
            if ruleset_rules.contains_key(&rule.name) {
                return Err(RuleErrors::DuplicateRule(rule.name.clone()).into());
            }

            ruleset_rules.insert(rule.name.clone(), ());
        }

        Ok(())
    }

    fn verify_called_rules(&self) -> Result<(), Box<dyn std::error::Error>> {
        for rule in &self.rules {
            for component in &rule.components {
                match component {
                    Component::RuleCall { name } => {
                        if !self.rules.iter().any(|rule| rule.name == *name) {
                            return Err(RuleErrors::CalleeUndefined(name.clone()).into());
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn verify_unused_rules(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut bitset = HashMap::new();

        for rule in &self.rules {
            bitset.insert(rule.name.clone(), false);
        }

        bitset.insert("program".to_string(), true);

        for rule in &self.rules {
            for component in &rule.components {
                match component {
                    Component::RuleCall { name } => {
                        bitset.insert(name.clone(), true);
                    }
                    _ => {}
                }
            }
        }

        let unused_rules: Vec<String> = bitset
            .into_iter()
            .filter(|(_, used)| !used)
            .map(|(name, _)| name)
            .collect();

        if unused_rules.is_empty() {
            Ok(())
        } else {
            return Err(RuleErrors::UnusedRules(unused_rules).into());
        }
    }

    fn verify_recursion(&self) -> Result<(), Box<dyn std::error::Error>> {
        for rule in &self.rules {
            for component in &rule.components {
                match component {
                    Component::RuleCall { name } => {
                        if rule.name == *name {
                            if rule.name != "program" {
                                return Err(RuleErrors::OnlyProgramIsAllowedRecusion.into());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

impl Component {
    pub fn from(token: Token<DuoKinds>) -> Result<Self, Box<dyn std::error::Error>> {
        match token.kind {
            DuoKinds::Regex | DuoKinds::RegexBlock => Ok(Self::Regex {
                regex: Regex::new(&token.value)?,
            }),
            DuoKinds::RuleCall => Ok(Self::RuleCall {
                name: token.value.to_string(),
            }),
            DuoKinds::Hardcoded => Ok(Self::Hardcoded {
                symbol: token.value.to_string(),
            }),
            DuoKinds::Alternative => Ok(Self::Alternative),
            _ => return Err(RuleErrors::NoParsingForRuleComponent.into()),
        }
    }
}
