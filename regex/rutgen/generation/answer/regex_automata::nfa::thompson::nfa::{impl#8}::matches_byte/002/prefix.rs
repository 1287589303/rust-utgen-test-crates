// Answer 0

#[test]
fn test_matches_byte_start_greater_than_byte() {
    let transition = Transition { start: 5, end: 10, next: StateID(SmallIndex::new(1)) };
    let byte = 3;
    transition.matches_byte(byte);
}

#[test]
fn test_matches_byte_start_greater_than_byte_with_end_equal() {
    let transition = Transition { start: 10, end: 10, next: StateID(SmallIndex::new(1)) };
    let byte = 9;
    transition.matches_byte(byte);
}

#[test]
fn test_matches_byte_start_greater_than_byte_with_large_end() {
    let transition = Transition { start: 20, end: 50, next: StateID(SmallIndex::new(1)) };
    let byte = 15;
    transition.matches_byte(byte);
}

