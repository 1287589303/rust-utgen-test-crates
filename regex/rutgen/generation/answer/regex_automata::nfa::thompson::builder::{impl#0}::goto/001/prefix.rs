// Answer 0

#[test]
fn test_goto_fail_state() {
    let state = State::Fail;
    let _ = state.goto();
}

#[test]
fn test_goto_capture_start() {
    let state = State::CaptureStart {
        pattern_id: PatternID(SmallIndex(0)),
        group_index: SmallIndex(1),
        next: StateID(SmallIndex(2)),
    };
    let _ = state.goto();
}

#[test]
fn test_goto_capture_end() {
    let state = State::CaptureEnd {
        pattern_id: PatternID(SmallIndex(1)),
        group_index: SmallIndex(0),
        next: StateID(SmallIndex(3)),
    };
    let _ = state.goto();
}

#[test]
fn test_goto_byte_range() {
    let state = State::ByteRange {
        trans: Transition { start: 0, end: 255, next: StateID(SmallIndex(4)) },
    };
    let _ = state.goto();
} 

#[test]
fn test_goto_spars() {
    let state = State::Sparse {
        transitions: vec![
            Transition { start: 1, end: 2, next: StateID(SmallIndex(5)) },
            Transition { start: 3, end: 4, next: StateID(SmallIndex(6)) },
        ],
    };
    let _ = state.goto();
}

