// Answer 0

#[test]
fn test_start_anchored_empty_pattern() {
    let nfa = NFA::new("").unwrap();
    let start_state = nfa.start_anchored();
}

#[test]
fn test_start_anchored_single_character() {
    let nfa = NFA::new("a").unwrap();
    let start_state = nfa.start_anchored();
}

#[test]
fn test_start_anchored_multiple_characters() {
    let nfa = NFA::new("abc").unwrap();
    let start_state = nfa.start_anchored();
}

#[test]
fn test_start_anchored_wildcard() {
    let nfa = NFA::new(".*").unwrap();
    let start_state = nfa.start_anchored();
}

#[test]
fn test_start_anchored_alternation() {
    let nfa = NFA::new("a|b").unwrap();
    let start_state = nfa.start_anchored();
}

#[test]
#[should_panic(expected = "BuildError")]
fn test_start_anchored_invalid_pattern() {
    let result = NFA::new("[").err();
}

