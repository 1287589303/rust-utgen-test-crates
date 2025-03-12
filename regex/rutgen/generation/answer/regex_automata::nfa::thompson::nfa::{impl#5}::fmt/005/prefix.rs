// Answer 0

#[test]
fn test_state_union_empty_alternates() {
    let alternates: Box<[StateID]> = Box::new([]);
    let state = State::Union { alternates };
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_union_single_alternate() {
    let single_alternate = StateID(SmallIndex::new_unchecked(1));
    let alternates: Box<[StateID]> = Box::new([single_alternate]);
    let state = State::Union { alternates };
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_union_multiple_alternates() {
    let alt1 = StateID(SmallIndex::new_unchecked(1));
    let alt2 = StateID(SmallIndex::new_unchecked(2));
    let alternates: Box<[StateID]> = Box::new([alt1, alt2]);
    let state = State::Union { alternates };
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_union_max_alternates() {
    let mut alternates_vec: Vec<StateID> = Vec::new();
    for i in 0..SmallIndex::LIMIT {
        alternates_vec.push(StateID(SmallIndex::new_unchecked(i)));
    }
    let alternates: Box<[StateID]> = alternates_vec.into_boxed_slice();
    let state = State::Union { alternates };
    let _ = format!("{:?}", state);
}

