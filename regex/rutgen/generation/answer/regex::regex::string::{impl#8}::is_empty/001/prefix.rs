// Answer 0

#[test]
fn test_is_empty_match_empty_start_equal_end() {
    let haystack = "abc";
    let start = 0;
    let end = 0;
    let m = Match::new(haystack, start, end);
    m.is_empty();
}

#[test]
fn test_is_empty_match_non_empty_case() {
    let haystack = "abc";
    let start = 0;
    let end = 1;
    let m = Match::new(haystack, start, end);
    m.is_empty();
}

#[test]
fn test_is_empty_match_full_range() {
    let haystack = "abc";
    let start = 0;
    let end = 3;
    let m = Match::new(haystack, start, end);
    m.is_empty();
}

#[test]
fn test_is_empty_match_middle_empty() {
    let haystack = "abc";
    let start = 1;
    let end = 1;
    let m = Match::new(haystack, start, end);
    m.is_empty();
} 

#[test]
fn test_is_empty_match_end_equals_length() {
    let haystack = "abc";
    let start = 3;
    let end = 3;
    let m = Match::new(haystack, start, end);
    m.is_empty();
} 

#[test]
fn test_is_empty_match_empty_haystack() {
    let haystack = "";
    let start = 0;
    let end = 0;
    let m = Match::new(haystack, start, end);
    m.is_empty();
} 

#[test]
fn test_is_empty_match_start_greater_length() {
    let haystack = "abc";
    let start = 4;
    let end = 4;
    let m = Match::new(haystack, start, end);
    m.is_empty();
} 

