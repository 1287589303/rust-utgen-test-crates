// Answer 0

#[test]
fn test_always_start_anchored_single_character() {
    let nfa = NFA::new("a").unwrap();
    nfa.is_always_start_anchored();
}

#[test]
fn test_always_start_anchored_anchored_pattern() {
    let nfa = NFA::new("^a").unwrap();
    nfa.is_always_start_anchored();
}

#[test]
fn test_always_start_anchored_multiline_pattern() {
    let nfa = NFA::new("(?m)^a").unwrap();
    nfa.is_always_start_anchored();
}

#[test]
fn test_always_start_anchored_mixed_pattern() {
    let nfa = NFA::new("(^a)|a").unwrap();
    nfa.is_always_start_anchored();
}

#[test]
fn test_always_start_anchored_multiple_anchored_patterns() {
    let nfa = NFA::new_many(&["^a", "^b", "^c"]).unwrap();
    nfa.is_always_start_anchored();
}

#[test]
fn test_always_start_anchored_multiple_patterns_with_unanchored() {
    let nfa = NFA::new_many(&["^a", "b", "^c"]).unwrap();
    nfa.is_always_start_anchored();
}

#[test]
fn test_always_start_anchored_empty_string() {
    let nfa = NFA::new("").unwrap();
    nfa.is_always_start_anchored();
}

#[test]
#[should_panic]
fn test_always_start_anchored_invalid_pattern() {
    let nfa = NFA::new("[").unwrap();
    nfa.is_always_start_anchored();
}

