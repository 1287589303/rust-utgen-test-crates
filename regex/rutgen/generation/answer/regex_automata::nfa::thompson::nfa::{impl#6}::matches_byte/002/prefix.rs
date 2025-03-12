// Answer 0

#[test]
fn test_matches_byte_with_exact_start() {
    let transition = Transition { start: 100, end: 150, next: StateID(1) };
    let transitions = SparseTransitions { transitions: Box::from([transition]) };
    let byte = 100;
    let result = transitions.matches_byte(byte);
}

#[test]
fn test_matches_byte_with_upper_bound() {
    let transition = Transition { start: 100, end: 150, next: StateID(2) };
    let transitions = SparseTransitions { transitions: Box::from([transition]) };
    let byte = 150;
    let result = transitions.matches_byte(byte);
}

#[test]
fn test_matches_byte_with_multiple_transitions() {
    let transition1 = Transition { start: 50, end: 75, next: StateID(3) };
    let transition2 = Transition { start: 100, end: 150, next: StateID(4) };
    let transitions = SparseTransitions { transitions: Box::from([transition1, transition2]) };
    let byte = 100;
    let result = transitions.matches_byte(byte);
}

