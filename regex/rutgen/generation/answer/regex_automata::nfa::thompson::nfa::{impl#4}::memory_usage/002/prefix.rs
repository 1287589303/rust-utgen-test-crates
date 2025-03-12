// Answer 0

#[test]
fn test_memory_usage_fail() {
    let state = State::Fail;
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_byte_range() {
    let transition = Transition { start: 0, end: 0, next: StateID(SmallIndex(0)) };
    let state = State::ByteRange { trans: transition };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_look() {
    let state_id = StateID(SmallIndex(0));
    let look = Look::Start;
    let state = State::Look { look, next: state_id };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_binary_union() {
    let state_id1 = StateID(SmallIndex(1));
    let state_id2 = StateID(SmallIndex(2));
    let state = State::BinaryUnion { alt1: state_id1, alt2: state_id2 };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_capture() {
    let state_id = StateID(SmallIndex(3));
    let pattern_id = PatternID(SmallIndex(1));
    let group_index = SmallIndex(0);
    let slot = SmallIndex(0);
    let state = State::Capture { next: state_id, pattern_id, group_index, slot };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_match() {
    let pattern_id = PatternID(SmallIndex(2));
    let state = State::Match { pattern_id };
    let _ = state.memory_usage();
}

