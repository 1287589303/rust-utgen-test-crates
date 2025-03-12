// Answer 0

#[test]
fn test_memory_usage_byte_range() {
    let state = State::ByteRange {
        trans: Transition { start: 0, end: 255, next: StateID(SmallIndex(0)) },
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_empty() {
    let state = State::Empty {
        next: StateID(SmallIndex(0)),
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_look() {
    let state = State::Look {
        look: Look::Start,
        next: StateID(SmallIndex(0)),
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_capture_start() {
    let state = State::CaptureStart {
        pattern_id: PatternID(SmallIndex(0)),
        group_index: SmallIndex(0),
        next: StateID(SmallIndex(0)),
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_capture_end() {
    let state = State::CaptureEnd {
        pattern_id: PatternID(SmallIndex(0)),
        group_index: SmallIndex(0),
        next: StateID(SmallIndex(0)),
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_fail() {
    let state = State::Fail;
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_match() {
    let state = State::Match {
        pattern_id: PatternID(SmallIndex(0)),
    };
    let _ = state.memory_usage();
}

