// Answer 0

#[test]
fn test_memory_usage_empty_state() {
    let state = State::Empty { next: StateID(SmallIndex(0)) };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_fail_state() {
    let state = State::Fail;
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_byte_range_state() {
    let transition = Transition { start: 0, end: 255, next: StateID(SmallIndex(1)) };
    let state = State::ByteRange { trans: transition };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_look_state() {
    let state = State::Look { look: Look::Start, next: StateID(SmallIndex(0)) };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_capture_start_state() {
    let state = State::CaptureStart { pattern_id: PatternID(SmallIndex(0)), group_index: SmallIndex(0), next: StateID(SmallIndex(1)) };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_capture_end_state() {
    let state = State::CaptureEnd { pattern_id: PatternID(SmallIndex(0)), group_index: SmallIndex(0), next: StateID(SmallIndex(1)) };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_match_state() {
    let state = State::Match { pattern_id: PatternID(SmallIndex(0)) };
    let _ = state.memory_usage();
}

