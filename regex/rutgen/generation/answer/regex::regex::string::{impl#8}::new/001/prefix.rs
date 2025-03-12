// Answer 0

#[test]
fn test_new_match_non_empty() {
    let haystack = "test";
    let start = 0;
    let end = 4;
    let match_result = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_empty_string() {
    let haystack = "";
    let start = 0;
    let end = 0;
    let match_result = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_start_equals_end() {
    let haystack = "sample";
    let start = 2;
    let end = 2;
    let match_result = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_full_range() {
    let haystack = "example";
    let start = 0;
    let end = 7; // haystack length is 7
    let match_result = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_substring() {
    let haystack = "substring";
    let start = 3;
    let end = 8; // selecting a substring within the range
    let match_result = Match::new(haystack, start, end);
}

#[test]
#[should_panic]
fn test_new_match_invalid_range_start_greater_than_end() {
    let haystack = "panic";
    let start = 3;
    let end = 2; // invalid range, should not compile
    let match_result = Match::new(haystack, start, end);
}

