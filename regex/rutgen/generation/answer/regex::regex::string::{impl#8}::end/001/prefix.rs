// Answer 0

#[test]
fn test_match_end_non_empty_string() {
    let haystack = "Hello, world!";
    let start = 0;
    let end = 13;
    let match_instance = Match::new(haystack, start, end);
    let _result = match_instance.end();
}

#[test]
fn test_match_end_within_substring() {
    let haystack = "Rust programming";
    let start = 0;
    let end = 4;
    let match_instance = Match::new(haystack, start, end);
    let _result = match_instance.end();
}

#[test]
fn test_match_end_empty_match() {
    let haystack = "Test string";
    let start = 5;
    let end = 5;
    let match_instance = Match::new(haystack, start, end);
    let _result = match_instance.end();
}

#[test]
fn test_match_end_single_character() {
    let haystack = "A";
    let start = 0;
    let end = 1;
    let match_instance = Match::new(haystack, start, end);
    let _result = match_instance.end();
}

#[test]
fn test_match_end_boundary_condition() {
    let haystack = "Boundary test";
    let start = 9;
    let end = 12; // End is on a valid UTF-8 boundary
    let match_instance = Match::new(haystack, start, end);
    let _result = match_instance.end();
}

#[test]
fn test_match_end_out_of_range() {
    let haystack = "Out of range";
    let start = 0;
    let end = 14; // This should be valid since it corresponds to the end of the string
    let match_instance = Match::new(haystack, start, end);
    let _result = match_instance.end();
}

