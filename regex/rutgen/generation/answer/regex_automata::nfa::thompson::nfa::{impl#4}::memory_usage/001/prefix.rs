// Answer 0

#[test]
fn test_memory_usage_match_state() {
    let state = State::Match {
        pattern_id: PatternID(SmallIndex(0)),
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_capture_state() {
    let state = State::Capture {
        next: StateID(SmallIndex(1)),
        pattern_id: PatternID(SmallIndex(2)),
        group_index: SmallIndex(0),
        slot: SmallIndex(0),
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_binary_union_state() {
    let state = State::BinaryUnion {
        alt1: StateID(SmallIndex(1)),
        alt2: StateID(SmallIndex(2)),
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_look_state() {
    let state = State::Look {
        look: Look::Start,
        next: StateID(SmallIndex(3)),
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_byte_range_state() {
    let state = State::ByteRange {
        trans: Transition {
            start: 0,
            end: 255,
            next: StateID(SmallIndex(4)),
        },
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_fail_state() {
    let state = State::Fail;
    let _ = state.memory_usage();
}

