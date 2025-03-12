// Answer 0

#[test]
fn test_match_as_str_valid_case() {
    let haystack = "Hello, World!";
    let match_instance = Match::new(haystack, 0, 5);
    let result = match_instance.as_str();
}

#[test]
fn test_match_as_str_empty_case() {
    let haystack = "";
    let match_instance = Match::new(haystack, 0, 0);
    let result = match_instance.as_str();
}

#[test]
fn test_match_as_str_full_range() {
    let haystack = "Full Range";
    let match_instance = Match::new(haystack, 0, haystack.len());
    let result = match_instance.as_str();
}

#[test]
fn test_match_as_str_substring_case() {
    let haystack = "Substrings are fun";
    let match_instance = Match::new(haystack, 0, 12);
    let result = match_instance.as_str();
}

#[test]
fn test_match_as_str_boundary_case() {
    let haystack = "Boundary test";
    let match_instance = Match::new(haystack, 9, 12);
    let result = match_instance.as_str();
}

#[test]
#[should_panic]
fn test_match_as_str_invalid_range_case() {
    let haystack = "Invalid range";
    let match_instance = Match::new(haystack, 5, 3);
    let result = match_instance.as_str();
}

