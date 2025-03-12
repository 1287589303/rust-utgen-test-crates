// Answer 0

#[test]
fn test_fmt_with_multiple_chunks() {
    let mut state = State {
        transitions: vec![
            Transition { start: 0, end: 1, next: StateID(1) },
            Transition { start: 2, end: 3, next: StateID(2) },
        ],
        chunks: vec![(0, 1), (1, 2)], // Two chunks, both non-empty
    };
    let mut buffer = vec![];
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    let _ = state.fmt(formatter);
}

#[test]
fn test_fmt_with_empty_chunk_second() {
    let mut state = State {
        transitions: vec![
            Transition { start: 0, end: 1, next: StateID(1) },
        ],
        chunks: vec![(0, 1), (1, 1)], // First chunk non-empty, second chunk empty
    };
    let mut buffer = vec![];
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    let _ = state.fmt(formatter);
}

#[test]
#[should_panic] // This test is expected to panic due to no valid write
fn test_fmt_with_invalid_writer() {
    let mut state = State {
        transitions: vec![
            Transition { start: 0, end: 1, next: StateID(1) },
            Transition { start: 2, end: 3, next: StateID(2) },
        ],
        chunks: vec![(0, 1), (1, 2)], // Two non-empty chunks
    };
    let mut buffer = vec![];
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    // Manipulate buffer to be invalid for writing
    buffer.clear();
    let _ = state.fmt(formatter);
}

