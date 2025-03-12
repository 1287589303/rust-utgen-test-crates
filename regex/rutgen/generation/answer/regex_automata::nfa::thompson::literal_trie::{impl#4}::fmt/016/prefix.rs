// Answer 0

#[test]
fn test_fmt_with_no_chunks() {
    let state = State::default();
    let mut buffer = Vec::new();
    let result = core::fmt::write(&mut buffer, |f| state.fmt(f));
}

#[test]
fn test_fmt_with_empty_transitions() {
    let mut state = State {
        transitions: vec![],
        chunks: vec![],
    };
    let mut buffer = Vec::new();
    let result = core::fmt::write(&mut buffer, |f| state.fmt(f));
}

#[test]
fn test_fmt_with_one_transition_in_active_chunk() {
    let mut state = State {
        transitions: vec![Transition { start: 1, end: 2, next: 1 }],
        chunks: vec![(0, 1)],
    };
    let mut buffer = Vec::new();
    let result = core::fmt::write(&mut buffer, |f| state.fmt(f));
}

#[test]
fn test_fmt_with_multiple_matches() {
    let mut state = State {
        transitions: vec![
            Transition { start: 1, end: 1, next: 1 },
            Transition { start: 2, end: 2, next: 1 },
        ],
        chunks: vec![(0, 2), (2, 3)],
    };
    let mut buffer = Vec::new();
    let result = core::fmt::write(&mut buffer, |f| state.fmt(f));
}

