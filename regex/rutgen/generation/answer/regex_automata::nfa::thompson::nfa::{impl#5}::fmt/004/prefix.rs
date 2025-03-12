// Answer 0

#[test]
fn test_state_binary_union_case_1() {
    let alt1 = StateID(SmallIndex::new_unchecked(0));
    let alt2 = StateID(SmallIndex::new_unchecked(1));
    let state = State::BinaryUnion { alt1, alt2 };
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_binary_union_case_2() {
    let alt1 = StateID(SmallIndex::new_unchecked(1));
    let alt2 = StateID(SmallIndex::new_unchecked(2));
    let state = State::BinaryUnion { alt1, alt2 };
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_binary_union_case_3() {
    let alt1 = StateID(SmallIndex::new_unchecked(SmallIndex::MAX.as_usize() - 1));
    let alt2 = StateID(SmallIndex::new_unchecked(SmallIndex::MAX.as_usize()));
    let state = State::BinaryUnion { alt1, alt2 };
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_binary_union_case_4() {
    let alt1 = StateID(SmallIndex::new_unchecked(0));
    let alt2 = StateID(SmallIndex::new_unchecked(SmallIndex::MAX.as_usize()));
    let state = State::BinaryUnion { alt1, alt2 };
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_binary_union_case_5() {
    let alt1 = StateID(SmallIndex::new_unchecked(SmallIndex::MAX.as_usize() - 1));
    let alt2 = StateID(SmallIndex::new_unchecked(0));
    let state = State::BinaryUnion { alt1, alt2 };
    let _ = format!("{:?}", state);
}

