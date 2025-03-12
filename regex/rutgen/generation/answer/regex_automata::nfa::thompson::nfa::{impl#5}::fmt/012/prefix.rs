// Answer 0

#[test]
fn test_state_dense_formatting_with_one_transition() {
    let state_id = StateID(SmallIndex::new_unchecked(1));
    let transitions = Box::new([state_id]);
    let dense = DenseTransitions { transitions };
    let state = State::Dense(dense);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", state);
}

#[test]
fn test_state_dense_formatting_with_empty_transitions() {
    let transitions: Box<[StateID]> = Box::new([]);
    let dense = DenseTransitions { transitions };
    let state = State::Dense(dense);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", state);
}

