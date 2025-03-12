// Answer 0

#[test]
fn test_add_union_state() {
    let mut inner = Inner::default();
    let alternate1 = StateID(SmallIndex::from(0));
    let alternate2 = StateID(SmallIndex::from(1));
    let state = State::Union {
        alternates: vec![alternate1, alternate2].into_boxed_slice(),
    };
    let _id = inner.add(state);
}

#[test]
fn test_add_union_state_empty() {
    let mut inner = Inner::default();
    let state = State::Union {
        alternates: Box::new([]),
    };
    let _id = inner.add(state);
}

