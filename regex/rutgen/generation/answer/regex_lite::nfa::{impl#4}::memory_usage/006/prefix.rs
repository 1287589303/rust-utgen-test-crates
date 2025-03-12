// Answer 0

#[test]
fn test_memory_usage_ranges_empty() {
    let state = State::Ranges {
        target: 1,
        ranges: vec![],
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_ranges_single_pair() {
    let state = State::Ranges {
        target: 1,
        ranges: vec![('a', 'b')],
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_ranges_multiple_pairs() {
    let state = State::Ranges {
        target: 1,
        ranges: vec![('a', 'b'), ('c', 'd'), ('e', 'f')],
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_ranges_boundary_characters() {
    let state = State::Ranges {
        target: 1,
        ranges: vec![('\u{0000}', '\u{007F}'), ('\u{0080}', '\u{FFFF}')],
    };
    let _ = state.memory_usage();
}

