// Answer 0

#[test]
fn test_matches_with_valid_index() {
    let transitions = DenseTransitions {
        transitions: Box::new([StateID(SmallIndex::from(1)); 256]),
    };
    let haystack = &[1, 2, 3];
    let at = 1;
    transitions.matches(haystack, at);
}

#[test]
fn test_matches_with_zero_length_haystack() {
    let transitions = DenseTransitions {
        transitions: Box::new([StateID(SmallIndex::from(1)); 256]),
    };
    let haystack: &[u8] = &[];
    let at = 0;
    transitions.matches(haystack, at);
}

#[test]
fn test_matches_with_out_of_bounds_index() {
    let transitions = DenseTransitions {
        transitions: Box::new([StateID(SmallIndex::from(1)); 256]),
    };
    let haystack = &[1, 2, 3];
    let at = haystack.len(); // Out of bounds
    transitions.matches(haystack, at);
}

#[test]
fn test_matches_with_empty_haystack() {
    let transitions = DenseTransitions {
        transitions: Box::new([StateID(SmallIndex::from(0)); 256]),
    };
    let haystack: &[u8] = &[];
    let at = 0; // Out of bounds for empty slice
    transitions.matches(haystack, at);
}

#[test]
fn test_matches_at_start_of_haystack() {
    let transitions = DenseTransitions {
        transitions: Box::new([StateID(SmallIndex::from(1)); 256]),
    };
    let haystack = &[0];
    let at = 0;
    transitions.matches(haystack, at);
}

#[test]
fn test_matches_at_end_of_haystack() {
    let transitions = DenseTransitions {
        transitions: Box::new([StateID(SmallIndex::from(1)); 256]),
    };
    let haystack = &[0, 1, 2];
    let at = haystack.len() - 1;
    transitions.matches(haystack, at);
}

