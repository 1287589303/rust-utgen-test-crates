// Answer 0

#[test]
fn test_add_dense_state() {
    let mut inner = Inner::default();
    let state = State::Dense { /* appropriate initialization */ };
    inner.add(state);
}

#[test]
#[should_panic]
fn test_add_exceed_state_limit() {
    let mut inner = Inner::default();
    for _ in 0..256 {
        let state = State::ByteRange { trans: Transition { start: 0, end: 0, next: StateID(SmallIndex(0)) } };
        inner.add(state);
    }
    let state = State::ByteRange { trans: Transition { start: 0, end: 0, next: StateID(SmallIndex(0)) } };
    inner.add(state);
}

