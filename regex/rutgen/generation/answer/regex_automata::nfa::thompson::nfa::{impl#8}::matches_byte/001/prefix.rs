// Answer 0

#[test]
fn test_matches_byte_with_start_equal_byte_and_end_greater() {
    let transition = Transition { start: 100, end: 200, next: StateID(0) };
    let result = transition.matches_byte(100);
}

#[test]
fn test_matches_byte_with_start_equal_byte_and_end_equal() {
    let transition = Transition { start: 150, end: 150, next: StateID(0) };
    let result = transition.matches_byte(150);
}

#[test]
fn test_matches_byte_with_start_equal_byte_and_end_less() {
    let transition = Transition { start: 200, end: 250, next: StateID(0) };
    let result = transition.matches_byte(200);
}

#[test]
fn test_matches_byte_with_start_at_min_and_byte_max() {
    let transition = Transition { start: 0, end: 255, next: StateID(0) };
    let result = transition.matches_byte(0);
}

#[test]
fn test_matches_byte_with_start_at_min_and_end_equal_to_max() {
    let transition = Transition { start: 0, end: 255, next: StateID(0) };
    let result = transition.matches_byte(255);
}

#[test]
fn test_matches_byte_with_start_middle_and_end_high() {
    let transition = Transition { start: 50, end: 255, next: StateID(0) };
    let result = transition.matches_byte(50);
}

#[test]
fn test_matches_byte_with_start_middle_and_end_low() {
    let transition = Transition { start: 100, end: 150, next: StateID(0) };
    let result = transition.matches_byte(100);
}

#[test]
fn test_matches_byte_with_start_gt_zero_and_byte_gt_start() {
    let transition = Transition { start: 50, end: 200, next: StateID(0) };
    let result = transition.matches_byte(51);
}

