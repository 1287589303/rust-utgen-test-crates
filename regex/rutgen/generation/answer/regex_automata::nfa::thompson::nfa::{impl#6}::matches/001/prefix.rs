// Answer 0

#[test]
fn test_matches_with_empty_haystack() {
    let transitions = SparseTransitions { transitions: Box::new([]) };
    let haystack: &[u8] = &[];
    let at = 0;
    transitions.matches(haystack, at);
}

#[test]
fn test_matches_with_single_byte_haystack() {
    let transitions = SparseTransitions {
        transitions: Box::new([Transition { start: 0, end: 255, next: StateID(0) }]),
    };
    let haystack: &[u8] = &[100];
    let at = 0;
    transitions.matches(haystack, at);
}

#[test]
fn test_matches_with_boundary_at_haystack_length() {
    let transitions = SparseTransitions { transitions: Box::new([]) };
    let haystack: &[u8] = &[1, 2, 3];
    let at = 3;
    transitions.matches(haystack, at);
}

#[test]
fn test_matches_with_haystack_length_of_one() {
    let transitions = SparseTransitions {
        transitions: Box::new([Transition { start: 0, end: 0, next: StateID(1) }]),
    };
    let haystack: &[u8] = &[0];
    let at = 0;
    transitions.matches(haystack, at);
}

#[test]
fn test_matches_with_max_byte_values() {
    let transitions = SparseTransitions {
        transitions: Box::new([Transition { start: 0, end: 255, next: StateID(1) }]),
    };
    let haystack: &[u8] = &[255];
    let at = 0;
    transitions.matches(haystack, at);
}

