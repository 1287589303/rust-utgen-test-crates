// Answer 0

#[test]
fn test_memory_usage_union_empty() {
    let state = State::Union { alternates: Vec::new() };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_union_single() {
    let state_id = StateID(SmallIndex(1));
    let state = State::Union { alternates: vec![state_id] };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_union_multiple() {
    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));
    let state_id_3 = StateID(SmallIndex(3));
    let state = State::Union { alternates: vec![state_id_1, state_id_2, state_id_3] };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_union_large() {
    let mut alternates = Vec::with_capacity(1000);
    for i in 0..1000 {
        alternates.push(StateID(SmallIndex(i)));
    }
    let state = State::Union { alternates };
    let _ = state.memory_usage();
}

