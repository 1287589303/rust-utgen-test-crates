// Answer 0

#[test]
fn test_as_str_valid_match() {
    let haystack = "Hello, World!";
    let start = 0;
    let end = 5;
    let match_instance = Match::new(haystack, start, end);
    match_instance.as_str();
}

#[test]
fn test_as_str_empty_match() {
    let haystack = "Hello, World!";
    let start = 5;
    let end = 5;
    let match_instance = Match::new(haystack, start, end);
    match_instance.as_str();
}

#[test]
fn test_as_str_full_haystack_match() {
    let haystack = "Hello, World!";
    let start = 0;
    let end = haystack.len();
    let match_instance = Match::new(haystack, start, end);
    match_instance.as_str();
}

#[test]
fn test_as_str_partial_match() {
    let haystack = "Hello, World!";
    let start = 7;
    let end = 12;
    let match_instance = Match::new(haystack, start, end);
    match_instance.as_str();
}

#[test]
#[should_panic]
fn test_as_str_start_greater_than_end() {
    let haystack = "Hello, World!";
    let start = 6;
    let end = 5;
    let match_instance = Match::new(haystack, start, end);
    match_instance.as_str();
}

#[test]
#[should_panic]
fn test_as_str_start_out_of_bounds() {
    let haystack = "Hello, World!";
    let start = 13;
    let end = 14;
    let match_instance = Match::new(haystack, start, end);
    match_instance.as_str();
}

#[test]
#[should_panic]
fn test_as_str_end_out_of_bounds() {
    let haystack = "Hello, World!";
    let start = 10;
    let end = 15;
    let match_instance = Match::new(haystack, start, end);
    match_instance.as_str();
}

