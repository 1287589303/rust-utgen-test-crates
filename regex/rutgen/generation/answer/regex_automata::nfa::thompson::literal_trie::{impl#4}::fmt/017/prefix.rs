// Answer 0

#[test]
fn test_fmt_with_empty_state() {
    let state = State {
        transitions: vec![],
        chunks: vec![(0, 1)], // At least one chunk to trigger self.chunks().enumerate()
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = state.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_single_chunk() {
    let transition = Transition { start: 0, end: 10, next: StateID::new(1) }; // Valid transition
    let state = State {
        transitions: vec![transition],
        chunks: vec![(0, 1)], // Single chunk
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = state.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_multiple_chunks() {
    let transition1 = Transition { start: 0, end: 10, next: StateID::new(1) }; // Valid transition
    let transition2 = Transition { start: 11, end: 20, next: StateID::new(2) }; // Valid transition
    let state = State {
        transitions: vec![transition1, transition2],
        chunks: vec![(0, 1), (1, 2)], // Multiple chunks
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = state.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_invalid_transition() {
    let transition = Transition { start: 255, end: 0, next: StateID::new(3) }; // Invalid transition
    let state = State {
        transitions: vec![transition],
        chunks: vec![(0, 1)], // At least one chunk
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = state.fmt(&mut buffer);
}

