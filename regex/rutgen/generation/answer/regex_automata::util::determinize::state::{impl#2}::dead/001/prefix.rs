// Answer 0

#[test]
fn test_dead_state() {
    let state = State::dead();
}

#[test]
fn test_dead_state_empty_builder() {
    let builder_empty = StateBuilderEmpty::new();
    let matches = builder_empty.into_matches();
    let nfa = matches.into_nfa();
    let state = nfa.to_state();
}

#[test]
fn test_dead_state_chained_operations() {
    let state = StateBuilderEmpty::new()
        .into_matches()
        .into_nfa()
        .to_state();
}

