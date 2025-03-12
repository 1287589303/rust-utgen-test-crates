// Answer 0

#[test]
fn test_memory_usage_dense() {
    let transitions: Box<[StateID]> = (0..256).map(|_| StateID(SmallIndex::default())).collect();
    let state = State::Dense(DenseTransitions { transitions });
    let _usage = state.memory_usage();
}

#[test]
fn test_memory_usage_dense_with_specific_states() {
    let transitions: Box<[StateID]> = (0..256).map(|i| StateID(SmallIndex(SmallIndex(i as u32)))).collect();
    let state = State::Dense(DenseTransitions { transitions });
    let _usage = state.memory_usage();
}

