// Answer 0

#[test]
fn test_is_epsilon_union() {
    let alternates: Box<[StateID]> = Box::new([StateID(SmallIndex::ZERO), StateID(SmallIndex(1))]);
    let state = State::Union { alternates };
    let result = state.is_epsilon();
}

#[test]
fn test_is_epsilon_union_multiple() {
    let alternates: Box<[StateID]> = Box::new([StateID(SmallIndex(2)), StateID(SmallIndex(3)), StateID(SmallIndex(4))]);
    let state = State::Union { alternates };
    let result = state.is_epsilon();
}

#[test]
fn test_is_epsilon_union_single_element() {
    let alternates: Box<[StateID]> = Box::new([StateID(SmallIndex(5))]);
    let state = State::Union { alternates };
    let result = state.is_epsilon();
}

