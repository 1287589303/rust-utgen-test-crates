// Answer 0

#[test]
fn test_matches_within_range() {
    let transition = Transition { start: 10, end: 20, next: StateID(SmallIndex(1)) };
    let haystack = &[15];  // A non-empty slice of bytes
    let at = 0;  // Within the bounds of haystack
    transition.matches(haystack, at);
}

#[test]
fn test_matches_boundary_lower() {
    let transition = Transition { start: 5, end: 10, next: StateID(SmallIndex(1)) };
    let haystack = &[5];  // A non-empty slice of bytes
    let at = 0;  // At the lower boundary (inclusive)
    transition.matches(haystack, at);
}

#[test]
fn test_matches_boundary_upper() {
    let transition = Transition { start: 5, end: 10, next: StateID(SmallIndex(1)) };
    let haystack = &[10];  // A non-empty slice of bytes
    let at = 0;  // At the upper boundary (inclusive)
    transition.matches(haystack, at);
}

#[test]
fn test_matches_out_of_range() {
    let transition = Transition { start: 20, end: 30, next: StateID(SmallIndex(1)) };
    let haystack = &[15];  // A non-empty slice of bytes
    let at = 0;  // At does not fall within the transition range
    transition.matches(haystack, at);
}

#[test]
fn test_matches_exceeds_haystack_length() {
    let transition = Transition { start: 5, end: 10, next: StateID(SmallIndex(1)) };
    let haystack = &[8];  // A non-empty slice of bytes
    let at = 1;  // At exceeds the length of haystack
    transition.matches(haystack, at);
}

