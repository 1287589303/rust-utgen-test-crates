// Answer 0

#[test]
fn test_memory_usage_with_empty_state() {
    let state = State::Empty { next: StateID(SmallIndex(0)) };
    state.memory_usage();
}

#[test]
fn test_memory_usage_with_byte_range() {
    let state = State::ByteRange { trans: Transition { start: 0, end: 255, next: StateID(SmallIndex(0)) } };
    state.memory_usage();
}

#[test]
fn test_memory_usage_with_look_state() {
    let state = State::Look { look: Look::Start, next: StateID(SmallIndex(0)) };
    state.memory_usage();
}

#[test]
fn test_memory_usage_with_capture_start() {
    let state = State::CaptureStart { pattern_id: PatternID(SmallIndex(0)), group_index: SmallIndex(0), next: StateID(SmallIndex(0)) };
    state.memory_usage();
}

#[test]
fn test_memory_usage_with_capture_end() {
    let state = State::CaptureEnd { pattern_id: PatternID(SmallIndex(0)), group_index: SmallIndex(0), next: StateID(SmallIndex(0)) };
    state.memory_usage();
}

#[test]
fn test_memory_usage_with_fail_state() {
    let state = State::Fail;
    state.memory_usage();
}

#[test]
fn test_memory_usage_with_match_state() {
    let state = State::Match { pattern_id: PatternID(SmallIndex(0)) };
    state.memory_usage();
}

