// Answer 0

#[test]
fn test_add_byte_range_valid() {
    let mut inner = Inner::default();
    let trans = Transition { start: 0, end: 10, next: StateID(0.into()) };
    let state = State::ByteRange { trans };
    let id = inner.add(state);
}

#[test]
fn test_add_byte_range_zero_range() {
    let mut inner = Inner::default();
    let trans = Transition { start: 0, end: 0, next: StateID(0.into()) };
    let state = State::ByteRange { trans };
    let id = inner.add(state);
}

#[test]
fn test_add_byte_range_full_range() {
    let mut inner = Inner::default();
    let trans = Transition { start: 0, end: 255, next: StateID(0.into()) };
    let state = State::ByteRange { trans };
    let id = inner.add(state);
}

#[test]
#[should_panic]
fn test_add_byte_range_invalid_range() {
    let mut inner = Inner::default();
    let trans = Transition { start: 10, end: 5, next: StateID(0.into()) }; // Invalid range
    let state = State::ByteRange { trans };
    let _id = inner.add(state);
}

