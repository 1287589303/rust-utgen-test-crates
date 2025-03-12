// Answer 0

#[test]
fn test_match_empty_haystack() {
    let haystack = "";
    let start = 0;
    let end = 0;
    let m = Match::new(haystack, start, end);
    let _ = m.is_empty();
}

#[test]
fn test_match_non_empty_string_zero_length() {
    let haystack = "hello";
    let start = 0;
    let end = 0;
    let m = Match::new(haystack, start, end);
    let _ = m.is_empty();
}

#[test]
fn test_match_non_empty_string_non_zero_length() {
    let haystack = "hello";
    let start = 0;
    let end = 5;
    let m = Match::new(haystack, start, end);
    let _ = m.is_empty();
}

#[test]
fn test_match_start_eq_end_large_values() {
    let haystack = "example";
    let start = 100;
    let end = 100;
    let m = Match::new(haystack, start, end);
    let _ = m.is_empty();
}

#[test]
fn test_match_start_less_than_end() {
    let haystack = "example";
    let start = 1;
    let end = 2;
    let m = Match::new(haystack, start, end);
    let _ = m.is_empty();
}

#[test]
fn test_match_start_greater_than_end() {
    let haystack = "example";
    let start = 2;
    let end = 1; // This would typically be an invalid case, but we can verify behavior
    let m = Match::new(haystack, start, end);
    let _ = m.is_empty();
}

#[test]
fn test_match_valid_zero_length_match() {
    let haystack = "test";
    let start = 4; // Start at the end of the string
    let end = 4; // Also the same point
    let m = Match::new(haystack, start, end);
    let _ = m.is_empty();
}

