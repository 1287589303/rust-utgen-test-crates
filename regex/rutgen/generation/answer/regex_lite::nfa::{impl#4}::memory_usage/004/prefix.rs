// Answer 0

#[test]
fn test_memory_usage_char() {
    let state = State::Char { target: 0, ch: 'a' };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_capture() {
    let state = State::Capture { target: 0, slot: 1 };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_fail() {
    let state = State::Fail;
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_match() {
    let state = State::Match;
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_goto() {
    let state = State::Goto { target: 1, look: None };
    let _ = state.memory_usage();
}

