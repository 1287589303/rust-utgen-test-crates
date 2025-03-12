// Answer 0

#[test]
fn test_range_valid_match() {
    let haystack: &[u8] = b"abcdef";
    let start = 1;
    let end = 4;
    let matched = Match::new(haystack, start, end);
    matched.range();
}

#[test]
fn test_range_empty_match() {
    let haystack: &[u8] = b"abcdef";
    let start = 2;
    let end = 2;
    let matched = Match::new(haystack, start, end);
    matched.range();
}

#[test]
fn test_range_start_equals_end() {
    let haystack: &[u8] = b"xyz";
    let start = 0;
    let end = 1; // Valid match
    let matched = Match::new(haystack, start, end);
    matched.range();
}

#[test]
fn test_range_boundary_case_start() {
    let haystack: &[u8] = b"hello";
    let start = 0;
    let end = 5; // end is at the length of haystack
    let matched = Match::new(haystack, start, end);
    matched.range();
}

#[test]
fn test_range_boundary_case_end() {
    let haystack: &[u8] = b"hello";
    let start = 4; // start is at the end - 1
    let end = 5;   // end is the length of haystack
    let matched = Match::new(haystack, start, end);
    matched.range();
}

#[test]
fn test_range_start_greater_than_zero() {
    let haystack: &[u8] = b"rustlang";
    let start = 2;
    let end = 6;
    let matched = Match::new(haystack, start, end);
    matched.range();
}

