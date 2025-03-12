// Answer 0

#[test]
fn test_goto_union_with_zero_alternates() {
    let state = State::Union { alternates: Vec::new() };
    let _result = state.goto();
}

#[test]
fn test_goto_union_with_multiple_alternates() {
    let state = State::Union { alternates: vec![StateID(SmallIndex(1)), StateID(SmallIndex(2))] };
    let _result = state.goto();
}

