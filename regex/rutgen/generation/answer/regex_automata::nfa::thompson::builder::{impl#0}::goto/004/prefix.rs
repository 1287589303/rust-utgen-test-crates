// Answer 0

#[test]
fn test_union_multiple_alternates() {
    use crate::util::primitives::SmallIndex;

    let state_id1 = StateID(SmallIndex(1));
    let state_id2 = StateID(SmallIndex(2));
    let state_id3 = StateID(SmallIndex(3));

    let alternates = vec![state_id1, state_id2, state_id3];

    let state = State::Union { alternates };

    let _ = state.goto();
}

#[test]
fn test_union_reverse_multiple_alternates() {
    use crate::util::primitives::SmallIndex;

    let state_id1 = StateID(SmallIndex(1));
    let state_id2 = StateID(SmallIndex(2));
    let state_id3 = StateID(SmallIndex(3));

    let alternates = vec![state_id1, state_id2, state_id3];

    let state = State::UnionReverse { alternates };

    let _ = state.goto();
}

