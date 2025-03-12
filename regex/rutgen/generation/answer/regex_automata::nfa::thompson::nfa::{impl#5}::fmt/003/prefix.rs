// Answer 0

#[test]
fn test_state_capture_fmt() {
    struct TestState {
        state: State,
    }

    let next_id = StateID(SmallIndex::new_unchecked(1));
    let pattern_id = PatternID(SmallIndex::new_unchecked(2));
    let group_index = SmallIndex::new_unchecked(0);
    let slot = SmallIndex::new_unchecked(1);

    let capture_state = State::Capture {
        next: next_id,
        pattern_id,
        group_index,
        slot,
    };

    let test_state = TestState { state: capture_state };

    let _ = format!("{:?}", test_state.state);
}

#[test]
fn test_state_capture_fmt_boundary() {
    struct TestState {
        state: State,
    }

    let next_id = StateID(SmallIndex::new_unchecked(1));
    let pattern_id = PatternID(SmallIndex::new_unchecked(2));
    let group_index = SmallIndex::new_unchecked(SmallIndex::LIMIT - 1);
    let slot = SmallIndex::new_unchecked(SmallIndex::LIMIT - 1);

    let capture_state = State::Capture {
        next: next_id,
        pattern_id,
        group_index,
        slot,
    };

    let test_state = TestState { state: capture_state };

    let _ = format!("{:?}", test_state.state);
}

