// Answer 0

#[test]
fn test_remap_fail_state_empty_remap() {
    let mut state = State::Fail;
    let remap: Vec<StateID> = Vec::new();
    state.remap(&remap);
}

#[test]
fn test_remap_fail_state_non_accessed_remap() {
    let mut state = State::Fail;
    let remap: Vec<StateID> = vec![StateID(SmallIndex(0))];
    state.remap(&remap);
}

