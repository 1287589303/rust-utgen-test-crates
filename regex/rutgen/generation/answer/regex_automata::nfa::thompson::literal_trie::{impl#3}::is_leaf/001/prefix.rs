// Answer 0

#[test]
fn test_is_leaf_empty_transitions() {
    let state = State::default();
    let result = state.is_leaf();
}

#[test]
fn test_is_leaf_non_empty_transitions() {
    let transition = Transition { start: 0, end: 255, next: StateID::from(1) };
    let state = State {
        transitions: vec![transition],
        chunks: Vec::new(),
    };
    let result = state.is_leaf();
}

