// Answer 0

#[test]
fn test_as_str_non_empty() {
    let haystack = "hello, world";
    let m = Match::new(haystack, 0, 5);
    let result = m.as_str();
}

#[test]
fn test_as_str_empty_match() {
    let haystack = "hello, world";
    let m = Match::new(haystack, 5, 5);
    let result = m.as_str();
}

#[test]
fn test_as_str_full_string_match() {
    let haystack = "hello, world";
    let m = Match::new(haystack, 0, haystack.len());
    let result = m.as_str();
}

#[test]
fn test_as_str_boundary_start() {
    let haystack = "hello, world";
    let m = Match::new(haystack, 0, 1);
    let result = m.as_str();
}

#[test]
fn test_as_str_boundary_end() {
    let haystack = "hello, world";
    let m = Match::new(haystack, haystack.len() - 1, haystack.len());
    let result = m.as_str();
}

