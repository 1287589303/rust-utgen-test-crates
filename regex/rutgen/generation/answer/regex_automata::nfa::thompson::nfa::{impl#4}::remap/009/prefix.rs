// Answer 0

#[test]
fn test_remap_dense_no_transitions() {
    let mut state = State::Dense(DenseTransitions {
        transitions: Box::new([]),
    });
    let remap: Vec<StateID> = vec![]; // Length 0
    state.remap(&remap);
}

#[test]
fn test_remap_dense_single_transition() {
    let state_id = StateID(SmallIndex(0));
    let mut state = State::Dense(DenseTransitions {
        transitions: Box::new([state_id; 256]),
    });
    let remap = vec![StateID(SmallIndex(1))]; // Length 1
    state.remap(&remap);
} 

#[test]
#[should_panic]
fn test_remap_dense_empty_remap() {
    let state_id = StateID(SmallIndex(0));
    let mut state = State::Dense(DenseTransitions {
        transitions: Box::new([state_id; 256]),
    });
    let remap: Vec<StateID> = vec![]; // Length 0
    state.remap(&remap);
}

