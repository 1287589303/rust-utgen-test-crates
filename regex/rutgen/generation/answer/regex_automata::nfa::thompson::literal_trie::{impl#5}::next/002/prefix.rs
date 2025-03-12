// Answer 0

#[test]
fn test_state_chunks_iter_next_valid_chunk() {
    let transitions = vec![
        Transition { start: 0, end: 5, next: StateID(1) },
        Transition { start: 5, end: 10, next: StateID(2) },
    ];

    let chunks: Vec<(usize, usize)> = vec![(0, 2)];
    let mut iter = StateChunksIter {
        transitions: &transitions,
        chunks: chunks.iter(),
        active: Some(&transitions[1..2]),
    };

    let result = iter.next();
}

#[test]
fn test_state_chunks_iter_next_with_active() {
    let transitions = vec![
        Transition { start: 1, end: 3, next: StateID(1) },
        Transition { start: 3, end: 5, next: StateID(2) },
    ];

    let chunks: Vec<(usize, usize)> = vec![(0, 1)];
    let mut iter = StateChunksIter {
        transitions: &transitions,
        chunks: chunks.iter(),
        active: Some(&transitions[0..1]),
    };

    let result = iter.next();
}

#[test]
fn test_state_chunks_iter_next_edge_case() {
    let transitions = vec![
        Transition { start: 0, end: 1, next: StateID(1) },
    ];

    let chunks: Vec<(usize, usize)> = vec![(0, 1)];
    let mut iter = StateChunksIter {
        transitions: &transitions,
        chunks: chunks.iter(),
        active: Some(&transitions[0..1]),
    };

    let result = iter.next();
}

#[test]
fn test_state_chunks_iter_next_active_only() {
    let transitions = vec![
        Transition { start: 8, end: 10, next: StateID(3) },
    ];

    let chunks: Vec<(usize, usize)> = vec![(0, 0)];
    let mut iter = StateChunksIter {
        transitions: &transitions,
        chunks: chunks.iter(),
        active: Some(&transitions[0..1]),
    };

    let result = iter.next();
}

