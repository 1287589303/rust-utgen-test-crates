// Answer 0

#[test]
fn test_match_as_str_valid_range() {
    let haystack = "Hello, world!";
    let start = 0;
    let end = 5;
    let m = Match::new(haystack, start, end);
    let result: &str = m.into();
}

#[test]
fn test_match_as_str_entire_haystack() {
    let haystack = "Hello, world!";
    let start = 0;
    let end = haystack.len();
    let m = Match::new(haystack, start, end);
    let result: &str = m.into();
}

#[test]
fn test_match_as_str_empty_match() {
    let haystack = "Hello, world!";
    let start = 5;
    let end = 5;
    let m = Match::new(haystack, start, end);
    let result: &str = m.into();
}

#[test]
fn test_match_as_str_substring() {
    let haystack = "Hello, world!";
    let start = 7;
    let end = 12;
    let m = Match::new(haystack, start, end);
    let result: &str = m.into();
}

#[test]
#[should_panic]
fn test_match_as_str_invalid_start() {
    let haystack = "Hello";
    let start = 6;
    let end = 6;
    let m = Match::new(haystack, start, end);
    let result: &str = m.into();
}

#[test]
#[should_panic]
fn test_match_as_str_invalid_end() {
    let haystack = "Hello";
    let start = 4;
    let end = 6;
    let m = Match::new(haystack, start, end);
    let result: &str = m.into();
}

#[test]
#[should_panic]
fn test_match_as_str_start_greater_than_end() {
    let haystack = "Hello";
    let start = 3;
    let end = 2;
    let m = Match::new(haystack, start, end);
    let result: &str = m.into();
}

