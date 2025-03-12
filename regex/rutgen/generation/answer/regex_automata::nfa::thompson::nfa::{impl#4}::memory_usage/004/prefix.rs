// Answer 0

#[test]
fn test_memory_usage_binary_union() {
    let state = State::BinaryUnion { alt1: StateID(SmallIndex(1)), alt2: StateID(SmallIndex(2)) };
    state.memory_usage();
}

#[test]
fn test_memory_usage_match() {
    let state = State::Match { pattern_id: PatternID(SmallIndex(1)) };
    state.memory_usage();
}

#[test]
fn test_memory_usage_capture() {
    let state = State::Capture { 
        next: StateID(SmallIndex(1)), 
        pattern_id: PatternID(SmallIndex(2)), 
        group_index: SmallIndex(0), 
        slot: SmallIndex(1) 
    };
    state.memory_usage();
}

#[test]
fn test_memory_usage_look() {
    let state = State::Look { 
        look: Look::Start, 
        next: StateID(SmallIndex(1)) 
    };
    state.memory_usage();
}

#[test]
fn test_memory_usage_byte_range() {
    let state = State::ByteRange {
        trans: Transition {
            start: 10,
            end: 20,
            next: StateID(SmallIndex(1)),
        },
    };
    state.memory_usage();
}

#[test]
fn test_memory_usage_fail() {
    let state = State::Fail;
    state.memory_usage();
}

