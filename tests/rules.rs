use codify::RuleSet;

const RULES_WORKING: &str = include_str!("../rules/working.rules");
const RULES_EMPTY_PROGRAM: &str = include_str!("../rules/empty_program.rules");
const RULES_WORKING_SHUFFLED: &str = include_str!("../rules/working_shuffled.rules");
const RULES_MISSING_PROGRAM: &str = include_str!("../rules/missing_program.rules");
const RULES_DUPLICATED: &str = include_str!("../rules/duplicated.rules");
const RULES_UNDEFINED_CALLEE: &str = include_str!("../rules/undefined_callee.rules");
const RULES_UNUSED: &str = include_str!("../rules/unused.rules");
const RULES_RECURSION: &str = include_str!("../rules/recursion.rules");

#[test]
fn working() {
    RuleSet::from(RULES_WORKING).unwrap();
}

#[test]
fn empty_program() {
    RuleSet::from(RULES_EMPTY_PROGRAM).unwrap();
}

#[test]
fn shuffled() {
    RuleSet::from(RULES_WORKING_SHUFFLED).unwrap();
}

#[test]
#[should_panic(expected = "EnforcedRuleUndefined")]
fn invalid() {
    RuleSet::from(RULES_MISSING_PROGRAM).unwrap();
}

#[test]
#[should_panic(expected = "DuplicateRule")]
fn duplicated() {
    RuleSet::from(RULES_DUPLICATED).unwrap();
}

#[test]
#[should_panic(expected = "CalleeUndefined")]
fn undefined_callee() {
    RuleSet::from(RULES_UNDEFINED_CALLEE).unwrap();
}

#[test]
#[should_panic(expected = "UnusedRule")]
fn unused() {
    RuleSet::from(RULES_UNUSED).unwrap();
}

#[test]
#[should_panic(expected = "OnlyProgramIsAllowedRecusion")]
fn recursion() {
    RuleSet::from(RULES_RECURSION).unwrap();
}
