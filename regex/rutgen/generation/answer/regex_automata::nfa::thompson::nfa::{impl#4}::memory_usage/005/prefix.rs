// Answer 0

#[test]
fn test_memory_usage_union_single_transition() {
    let state_id = StateID(SmallIndex(1));
    let state = State::Union {
        alternates: Box::new([state_id]),
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_union_multiple_transitions() {
    let state_ids: Box<[StateID]> = Box::new([
        StateID(SmallIndex(1)),
        StateID(SmallIndex(2)),
        StateID(SmallIndex(3)),
    ]);
    let state = State::Union { alternates: state_ids };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_union_max_length() {
    let state_ids: Box<[StateID]> = (0..256).map(|i| StateID(SmallIndex(i))).collect();
    let state = State::Union { alternates: state_ids };
    let _ = state.memory_usage();
}

