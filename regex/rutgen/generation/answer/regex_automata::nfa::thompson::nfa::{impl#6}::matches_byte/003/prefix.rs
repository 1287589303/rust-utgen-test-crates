// Answer 0

#[test]
fn test_matches_byte_exact_start_false() {
    let transition = Transition { start: 5, end: 10, next: StateID(1) };
    let transitions = Box::new([transition]);

    let sparse_transitions = SparseTransitions { transitions };

    let result = sparse_transitions.matches_byte(5);
}

#[test]
fn test_matches_byte_exact_start_false_no_matches() {
    let transition1 = Transition { start: 5, end: 10, next: StateID(1) };
    let transition2 = Transition { start: 11, end: 15, next: StateID(2) };
    let transitions = Box::new([transition1, transition2]);

    let sparse_transitions = SparseTransitions { transitions };

    let result = sparse_transitions.matches_byte(5);
}

#[test]
fn test_matches_byte_exact_start_false_multiple_transitions() {
    let transition1 = Transition { start: 7, end: 8, next: StateID(1) };
    let transition2 = Transition { start: 9, end: 12, next: StateID(2) };
    let transition3 = Transition { start: 5, end: 6, next: StateID(3) };
    let transitions = Box::new([transition3, transition1, transition2]);

    let sparse_transitions = SparseTransitions { transitions };

    let result = sparse_transitions.matches_byte(5);
}

