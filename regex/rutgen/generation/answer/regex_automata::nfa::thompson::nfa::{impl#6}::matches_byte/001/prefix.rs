// Answer 0

#[test]
fn test_matches_byte_less_than_start_of_first_transition() {
    let transition1 = Transition { start: 5, end: 10, next: StateID(1) };
    let transition2 = Transition { start: 15, end: 20, next: StateID(2) };
    let sparse_transitions = SparseTransitions {
        transitions: Box::new([transition1, transition2]),
    };
    let byte = 4; // byte < min(t.start)
    let result = sparse_transitions.matches_byte(byte);
}

#[test]
fn test_matches_byte_less_than_start_of_only_transition() {
    let transition = Transition { start: 8, end: 12, next: StateID(3) };
    let sparse_transitions = SparseTransitions {
        transitions: Box::new([transition]),
    };
    let byte = 7; // byte < min(t.start)
    let result = sparse_transitions.matches_byte(byte);
}

#[test]
fn test_matches_byte_less_than_start_of_multiple_transitions() {
    let transition1 = Transition { start: 10, end: 15, next: StateID(4) };
    let transition2 = Transition { start: 20, end: 25, next: StateID(5) };
    let sparse_transitions = SparseTransitions {
        transitions: Box::new([transition1, transition2]),
    };
    let byte = 9; // byte < min(t.start)
    let result = sparse_transitions.matches_byte(byte);
}

