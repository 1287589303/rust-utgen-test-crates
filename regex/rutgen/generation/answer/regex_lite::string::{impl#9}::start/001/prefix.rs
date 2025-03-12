// Answer 0

#[test]
fn test_match_start_zero_offset() {
    let haystack = "test";
    let m = Match::new(haystack, 0, 4);
    m.start();
}

#[test]
fn test_match_start_mid_offset() {
    let haystack = "test";
    let m = Match::new(haystack, 2, 4);
    m.start();
}

#[test]
fn test_match_start_end_offset() {
    let haystack = "test";
    let m = Match::new(haystack, 4, 4);
    m.start();
}

#[test]
fn test_match_start_large_offset() {
    let haystack = "test string";
    let m = Match::new(haystack, 10, 10);
    m.start();
}

#[test]
fn test_match_start_boundary_condition() {
    let haystack = "hello";
    let m = Match::new(haystack, 0, 5);
    m.start();
}

#[test]
fn test_match_start_offset_equal_to_haystack_length() {
    let haystack = "sample";
    let m = Match::new(haystack, 6, 6);
    m.start();
}

