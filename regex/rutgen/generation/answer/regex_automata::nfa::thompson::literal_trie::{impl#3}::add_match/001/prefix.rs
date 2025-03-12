// Answer 0

#[test]
fn test_add_match_no_transitions_no_chunks() {
    let mut state = State::default();
    state.add_match();
}

#[test]
fn test_add_match_no_transitions_existing_chunks() {
    let mut state = State {
        chunks: vec![(0, 0)],
        ..State::default()
    };
    state.add_match();
}

#[test]
fn test_add_match_existing_transitions_no_chunks() {
    let mut state = State {
        transitions: vec![Transition { start: 0, end: 0, next: StateID::default() }],
        ..State::default()
    };
    state.add_match();
}

