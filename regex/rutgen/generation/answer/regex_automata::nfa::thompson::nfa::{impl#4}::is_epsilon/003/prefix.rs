// Answer 0

#[test]
fn test_is_epsilon_capture() {
    let state = State::Capture {
        next: StateID::default(),
        pattern_id: PatternID::default(),
        group_index: SmallIndex::default(),
        slot: SmallIndex::default(),
    };
    let _result = state.is_epsilon();
}

#[test]
fn test_is_epsilon_capture_non_default() {
    let state = State::Capture {
        next: StateID(SmallIndex(1)),
        pattern_id: PatternID(SmallIndex(1)),
        group_index: SmallIndex(2),
        slot: SmallIndex(3),
    };
    let _result = state.is_epsilon();
}

