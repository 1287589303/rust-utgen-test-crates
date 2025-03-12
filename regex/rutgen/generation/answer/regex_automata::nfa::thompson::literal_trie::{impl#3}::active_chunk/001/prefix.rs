// Answer 0

#[test]
fn test_active_chunk_with_non_empty_transitions_start_zero() {
    let mut state = State {
        transitions: vec![Transition { start: 1, end: 3, next: StateID(0) }],
        chunks: Vec::new(),
    };
    let _ = state.active_chunk();
}

#[test]
fn test_active_chunk_with_non_empty_transitions_boundary_start() {
    let mut state = State {
        transitions: vec![
            Transition { start: 1, end: 3, next: StateID(0) },
            Transition { start: 4, end: 6, next: StateID(1) },
        ],
        chunks: Vec::new(),
    };
    state.chunks.push((0, 1));
    let _ = state.active_chunk();
}

#[test]
fn test_active_chunk_with_two_transitions() {
    let mut state = State {
        transitions: vec![
            Transition { start: 1, end: 3, next: StateID(0) },
            Transition { start: 4, end: 6, next: StateID(1) },
        ],
        chunks: Vec::new(),
    };
    state.chunks.push((0, 2));
    let _ = state.active_chunk();
}

#[test]
fn test_active_chunk_with_non_empty_transitions_start_equals_length() {
    let mut state = State {
        transitions: vec![Transition { start: 1, end: 3, next: StateID(0) }],
        chunks: Vec::new(),
    };
    state.chunks.push((0, 1));
    let _ = state.active_chunk(); // Tests start equal to transitions length
}

#[test]
fn test_active_chunk_with_different_sizes() {
    let mut state = State {
        transitions: Vec::new(),
        chunks: Vec::new(),
    };
    state.transitions.push(Transition { start: 1, end: 3, next: StateID(0) });
    state.chunks.push((0, 1));
    let _ = state.active_chunk();
}

