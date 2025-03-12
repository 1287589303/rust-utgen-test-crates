// Answer 0

#[test]
fn test_fmt_with_unprintable_bytes() {
    let state_id1 = StateID(SmallIndex::new(0));
    let state_id2 = StateID(SmallIndex::new(1));
    let state_id3 = StateID(SmallIndex::new(2));
    let unprintable_state = State {
        transitions: vec![],
    };
    let valid_state = State {
        transitions: vec![],
    };

    let inner = Inner {
        states: vec![unprintable_state.clone(), valid_state.clone(), valid_state],
        start_anchored: state_id1,
        start_unanchored: state_id2,
        start_pattern: vec![state_id3],
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };

    let mut buffer = String::new();
    let formatter = &mut buffer;

    let result = inner.fmt(formatter);
}

#[test]
fn test_fmt_with_diverse_states() {
    let state_id1 = StateID(SmallIndex::new(0));
    let state_id2 = StateID(SmallIndex::new(1));
    let state_id3 = StateID(SmallIndex::new(2));

    let valid_state = State {
        transitions: vec![],
    };
    let invalid_state = State {
        transitions: vec![],
    };

    let inner = Inner {
        states: vec![valid_state.clone(), invalid_state.clone(), valid_state],
        start_anchored: state_id1,
        start_unanchored: state_id2,
        start_pattern: vec![state_id3],
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };

    let mut buffer = String::new();
    let formatter = &mut buffer;

    let result = inner.fmt(formatter);
}

#[test]
fn test_fmt_with_invalid_state_id() {
    let state_id1 = StateID(SmallIndex::new(2)); // out of bounds if `states` has less than 3
    let state_id2 = StateID(SmallIndex::new(1));
    let state_id3 = StateID(SmallIndex::new(0));

    let state = State {
        transitions: vec![],
    };

    let inner = Inner {
        states: vec![state.clone(), state.clone()],
        start_anchored: state_id1,
        start_unanchored: state_id2,
        start_pattern: vec![state_id3],
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };

    let mut buffer = String::new();
    let formatter = &mut buffer;

    let result = inner.fmt(formatter);
}

