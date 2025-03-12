// Answer 0

#[test]
fn test_match_range_valid_case() {
    let haystack = "Hello, World!";
    let start = 0;
    let end = 5;
    let m = Match::new(haystack, start, end);
    let _range: core::ops::Range<usize> = m.into();
}

#[test]
fn test_match_range_empty_case() {
    let haystack = "Hello, World!";
    let start = 5;
    let end = 5;
    let m = Match::new(haystack, start, end);
    let _range: core::ops::Range<usize> = m.into();
}

#[test]
fn test_match_range_full_case() {
    let haystack = "Hello, World!";
    let start = 0;
    let end = haystack.len();
    let m = Match::new(haystack, start, end);
    let _range: core::ops::Range<usize> = m.into();
}

#[test]
fn test_match_range_out_of_bounds_start() {
    let haystack = "Hello, World!";
    let start = 14; // Out of bounds since haystack.len() is 13
    let end = 14;
    let m = Match::new(haystack, start, end);
    let _range: core::ops::Range<usize> = m.into(); // This case is expected to panic
}

#[test]
fn test_match_range_out_of_bounds_end() {
    let haystack = "Hello, World!";
    let start = 0;
    let end = 15; // Out of bounds since haystack.len() is 13
    let m = Match::new(haystack, start, end);
    let _range: core::ops::Range<usize> = m.into(); // This case is expected to panic
}

#[test]
fn test_match_range_reverse_case() {
    let haystack = "Hello, World!";
    let start = 5;
    let end = 3; // end < start
    let m = Match::new(haystack, start, end);
    let _range: core::ops::Range<usize> = m.into(); // This case is expected to panic
}

