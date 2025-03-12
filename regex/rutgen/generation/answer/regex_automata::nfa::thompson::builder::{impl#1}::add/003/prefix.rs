// Answer 0

#[test]
fn test_add_empty_state() {
    let mut builder = Builder::new();
    let empty_state = State::Empty {
        next: StateID::default(),
    };
    let _ = builder.add(empty_state);
}

#[test]
fn test_add_byte_range_state() {
    let mut builder = Builder::new();
    let byte_range_state = State::ByteRange {
        trans: Transition::default(),
    };
    let _ = builder.add(byte_range_state);
}

#[test]
fn test_add_sparse_state() {
    let mut builder = Builder::new();
    let sparse_state = State::Sparse {
        transitions: vec![Transition::default(), Transition::default()],
    };
    let _ = builder.add(sparse_state);
}

#[test]
fn test_add_union_state() {
    let mut builder = Builder::new();
    let union_state = State::Union {
        alternates: vec![StateID::default(), StateID::default()],
    };
    let _ = builder.add(union_state);
}

#[test]
fn test_add_fail_state() {
    let mut builder = Builder::new();
    let fail_state = State::Fail;
    let _ = builder.add(fail_state);
}

#[test]
fn test_add_match_state() {
    let mut builder = Builder::new();
    let match_state = State::Match {
        pattern_id: PatternID::default(),
    };
    let _ = builder.add(match_state);
}

