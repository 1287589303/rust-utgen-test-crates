// Answer 0

#[test]
fn test_new_match_valid_range() {
    let haystack = "Hello, world!";
    let start = 0;
    let end = haystack.len();
    let m = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_start_equal_end() {
    let haystack = "Test string";
    let start = 5;
    let end = start;
    let m = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_start_greater_than_end() {
    let haystack = "Another test";
    let start = 8;
    let end = 5; // intentionally invalid
    let m = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_start_at_length() {
    let haystack = "Boundary test";
    let start = haystack.len();
    let end = start;
    let m = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_end_at_length() {
    let haystack = "Another boundary";
    let start = 0;
    let end = haystack.len();
    let m = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_empty_haystack() {
    let haystack = "";
    let start = 0;
    let end = 0;
    let m = Match::new(haystack, start, end);
}

