// Answer 0

#[test]
fn test_range_valid_offsets() {
    let haystack = "hello";
    let start = 1;
    let end = 4;
    let match_obj = Match::new(haystack, start, end);
    let _ = match_obj.range();
}

#[test]
fn test_range_empty_match() {
    let haystack = "test";
    let start = 0;
    let end = 0;
    let match_obj = Match::new(haystack, start, end);
    let _ = match_obj.range();
}

#[test]
fn test_range_full_haystack() {
    let haystack = "example";
    let start = 0;
    let end = 7;
    let match_obj = Match::new(haystack, start, end);
    let _ = match_obj.range();
}

#[test]
fn test_range_single_character() {
    let haystack = "world";
    let start = 2;
    let end = 3;
    let match_obj = Match::new(haystack, start, end);
    let _ = match_obj.range();
}

#[test]
fn test_range_boundary_conditions() {
    let haystack = "boundary";
    let start = 0;
    let end = 8;
    let match_obj = Match::new(haystack, start, end);
    let _ = match_obj.range();
}

#[test]
#[should_panic]
fn test_range_invalid_start() {
    let haystack = "panic";
    let start = 5; // Invalid since it exceeds haystack length
    let end = 6;
    let match_obj = Match::new(haystack, start, end);
    let _ = match_obj.range();
}

#[test]
#[should_panic]
fn test_range_invalid_end() {
    let haystack = "check";
    let start = 1;
    let end = 6; // Invalid since it exceeds haystack length
    let match_obj = Match::new(haystack, start, end);
    let _ = match_obj.range();
}

