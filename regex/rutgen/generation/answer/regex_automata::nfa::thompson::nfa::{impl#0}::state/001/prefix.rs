// Answer 0

#[test]
fn test_state_valid_id_zero() {
    let nfa = NFA::new("a").unwrap();
    let valid_id = StateID(SmallIndex::from(0));
    let state = nfa.state(valid_id);
}

#[test]
fn test_state_valid_id_middle() {
    let nfa = NFA::new("a|b").unwrap();
    let valid_id = StateID(SmallIndex::from(1)); // Assuming at least two states exist
    let state = nfa.state(valid_id);
}

#[test]
fn test_state_valid_id_last() {
    let nfa = NFA::new("abc").unwrap();
    let valid_id = StateID(SmallIndex::from(nfa.states().len() as usize - 1));
    let state = nfa.state(valid_id);
}

#[should_panic]
fn test_state_invalid_id_too_high() {
    let nfa = NFA::new("a").unwrap();
    let invalid_id = StateID(SmallIndex::from(nfa.states().len()));
    let _state = nfa.state(invalid_id);
}

