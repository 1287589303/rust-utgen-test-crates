// Answer 0

#[test]
fn test_empty_regex() {
    let nfa = NFA::new("").unwrap();
    nfa.has_empty();
}

#[test]
fn test_repetition_plus() {
    let nfa = NFA::new("a+").unwrap();
    nfa.has_empty();
}

#[test]
fn test_repetition_star() {
    let nfa = NFA::new("a*").unwrap();
    nfa.has_empty();
}

#[test]
fn test_nested_repetition() {
    let nfa = NFA::new("(a+)*").unwrap();
    nfa.has_empty();
}

#[test]
fn test_look_around() {
    let nfa = NFA::compiler()
        .syntax(syntax::Config::new().utf8(false))
        .build(r"^$\A\z\b\B(?-u:\b\B)").unwrap();
    nfa.has_empty();
}

#[test]
fn test_look_around_plus() {
    let nfa = NFA::new(r"\b+").unwrap();
    nfa.has_empty();
}

#[test]
fn test_alternation() {
    let nfa = NFA::new("foo|(bar)?|quux").unwrap();
    nfa.has_empty();
}

#[test]
fn test_never_matching() {
    let nfa = NFA::new("[a&&b]").unwrap();
    nfa.has_empty();
}

#[test]
fn test_never_match_star() {
    let nfa = NFA::new("[a&&b]*").unwrap();
    nfa.has_empty();
}

#[test]
fn test_never_match_plus() {
    let nfa = NFA::new("[a&&b]+").unwrap();
    nfa.has_empty();
}

