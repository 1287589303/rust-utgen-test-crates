// Answer 0

#[test]
fn test_remap_dense_transitions_valid() {
    let mut state = State::Dense(DenseTransitions {
        transitions: Box::new((0..256).map(|i| StateID(SmallIndex(i))).collect::<Vec<_>>().into_boxed_slice()),
    });
    let remap: Vec<StateID> = (0..256).map(|i| StateID(SmallIndex(i + 1))).collect();
    state.remap(&remap);
}

#[test]
fn test_remap_dense_transitions_partial() {
    let mut state = State::Dense(DenseTransitions {
        transitions: Box::new((0..256).map(|i| StateID(SmallIndex(i))).collect::<Vec<_>>().into_boxed_slice()),
    });
    let remap: Vec<StateID> = vec![StateID(SmallIndex(1)); 128]; // only mapping first half
    state.remap(&remap);
}

#[test]
fn test_remap_dense_transitions_empty() {
    let mut state = State::Dense(DenseTransitions {
        transitions: Box::new((0..256).map(|i| StateID(SmallIndex(i))).collect::<Vec<_>>().into_boxed_slice()),
    });
    let remap: Vec<StateID> = vec![]; // empty remap slice
    state.remap(&remap);
}

#[test]
fn test_remap_dense_transitions_noop() {
    let mut state = State::Dense(DenseTransitions {
        transitions: Box::new((0..256).map(|i| StateID(SmallIndex(i))).collect::<Vec<_>>().into_boxed_slice()),
    });
    let remap: Vec<StateID> = (0..256).map(|i| StateID(SmallIndex(i))).collect(); // identity mapping
    state.remap(&remap);
}

