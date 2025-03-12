// Answer 0

#[test]
fn test_new_match_valid() {
    let haystack = b"hello world";
    let start = 0;
    let end = 5;
    let match_result = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_empty() {
    let haystack = b"";
    let start = 0;
    let end = 0;
    let match_result = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_start_equals_end() {
    let haystack = b"test string";
    let start = 4;
    let end = 4;
    let match_result = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_full_range() {
    let haystack = b"full range test";
    let start = 0;
    let end = haystack.len();
    let match_result = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_invalid_start() {
    let haystack = b"invalid";
    let start = 8; // out of bounds
    let end = 8; 
    let match_result = Match::new(haystack, start, end);
}

#[test]
fn test_new_match_invalid_range() {
    let haystack = b"invalid range";
    let start = 5;
    let end = 4; // end < start
    let match_result = Match::new(haystack, start, end);
}

