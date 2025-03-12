// Answer 0

#[test]
fn test_matches_byte_no_transitions_case() {
    let transitions: Box<[Transition]> = Box::new([]);
    let sparse_transitions = SparseTransitions { transitions };

    let result = sparse_transitions.matches_byte(5);
}

#[test]
fn test_matches_byte_below_min_transition_start() {
    let transitions: Box<[Transition]> = Box::new([
        Transition { start: 10, end: 20, next: StateID(1) },
        Transition { start: 30, end: 40, next: StateID(2) },
    ]);
    let sparse_transitions = SparseTransitions { transitions };

    let result = sparse_transitions.matches_byte(5);
}

#[test]
fn test_matches_byte_boundary_case() {
    let transitions: Box<[Transition]> = Box::new([
        Transition { start: 10, end: 20, next: StateID(1) },
    ]);
    let sparse_transitions = SparseTransitions { transitions };

    let result = sparse_transitions.matches_byte(9);
}

