// Answer 0

#[test]
fn test_goto_union_reverse_multiple() {
    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));
    let state_id_3 = StateID(SmallIndex(3));
    
    let state = State::UnionReverse {
        alternates: vec![state_id_1, state_id_2, state_id_3],
    };

    let _ = state.goto();
}

#[test]
fn test_goto_union_reverse_empty() {
    let state = State::UnionReverse {
        alternates: vec![],
    };

    let _ = state.goto();
}

