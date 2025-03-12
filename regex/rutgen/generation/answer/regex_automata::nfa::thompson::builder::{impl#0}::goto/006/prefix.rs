// Answer 0

#[test]
fn test_goto_empty_state() {
    let next_state_id = StateID(SmallIndex(1));
    let state = State::Empty { next: next_state_id };
    let result = state.goto();
}

#[test]
fn test_goto_union_state() {
    let next_state_id = StateID(SmallIndex(2));
    let state = State::Union { alternates: vec![next_state_id] };
    let result = state.goto();
}

#[test]
fn test_goto_union_reverse_state() {
    let next_state_id = StateID(SmallIndex(3));
    let state = State::UnionReverse { alternates: vec![next_state_id] };
    let result = state.goto();
}

