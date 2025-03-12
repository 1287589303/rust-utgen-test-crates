// Answer 0

#[test]
fn test_empty_chunks() {
    let state = State::default();
    let mut formatter = core::fmt::Formatter::new();
    let _ = state.fmt(&mut formatter);
}

#[test]
fn test_single_chunk_single_transition() {
    let mut state = State::default();
    state.chunks.push((0, 1));
    state.transitions.push(Transition { byte: 1, next: StateID::default() });
    let mut formatter = core::fmt::Formatter::new();
    let _ = state.fmt(&mut formatter);
}

#[test]
fn test_single_chunk_multiple_transitions() {
    let mut state = State::default();
    state.chunks.push((0, 3));
    state.transitions.push(Transition { byte: 1, next: StateID::default() });
    state.transitions.push(Transition { byte: 2, next: StateID::default() });
    state.transitions.push(Transition { byte: 3, next: StateID::default() });
    let mut formatter = core::fmt::Formatter::new();
    let _ = state.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_error_formatting() {
    let mut state = State::default();
    state.chunks.push((0, 2));
    state.transitions.push(Transition { byte: 256, next: StateID::default() });
    state.transitions.push(Transition { byte: 257, next: StateID::default() });
    let mut formatter = core::fmt::Formatter::new();
    let _ = state.fmt(&mut formatter);
}

