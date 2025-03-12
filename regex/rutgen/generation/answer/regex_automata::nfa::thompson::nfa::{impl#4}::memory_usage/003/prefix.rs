// Answer 0

#[test]
fn test_memory_usage_capture() {
    let state = State::Capture {
        next: StateID(SmallIndex(0)),
        pattern_id: PatternID(SmallIndex(0)),
        group_index: SmallIndex(0),
        slot: SmallIndex(0),
    };
    let _memory_usage = state.memory_usage();
}

#[test]
fn test_memory_usage_binary_union() {
    let state = State::BinaryUnion {
        alt1: StateID(SmallIndex(0)),
        alt2: StateID(SmallIndex(1)),
    };
    let _memory_usage = state.memory_usage();
}

#[test]
fn test_memory_usage_match() {
    let state = State::Match {
        pattern_id: PatternID(SmallIndex(0)),
    };
    let _memory_usage = state.memory_usage();
}

#[test]
fn test_memory_usage_look() {
    let state = State::Look {
        look: Look::Start,
        next: StateID(SmallIndex(0)),
    };
    let _memory_usage = state.memory_usage();
}

#[test]
fn test_memory_usage_byte_range() {
    let state = State::ByteRange {
        trans: Transition {
            start: 0,
            end: 255,
            next: StateID(SmallIndex(0)),
        },
    };
    let _memory_usage = state.memory_usage();
}

#[test]
fn test_memory_usage_fail() {
    let state = State::Fail;
    let _memory_usage = state.memory_usage();
}

