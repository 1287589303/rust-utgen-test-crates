// Answer 0

#[test]
fn test_match_debug_format_non_empty_haystack() {
    let haystack = "Hello, world!";
    let start = 0;
    let end = haystack.len();
    let m = Match::new(haystack, start, end);
    let _ = core::fmt::format(format_args!("{:?}", m));
}

#[test]
fn test_match_debug_format_start_equal_end() {
    let haystack = "Hello, world!";
    let start = 5;
    let end = 5;
    let m = Match::new(haystack, start, end);
    let _ = core::fmt::format(format_args!("{:?}", m));
}

#[test]
fn test_match_debug_format_single_character_match() {
    let haystack = "Hello, world!";
    let start = 7;
    let end = 8;
    let m = Match::new(haystack, start, end);
    let _ = core::fmt::format(format_args!("{:?}", m));
}

#[test]
fn test_match_debug_format_full_string() {
    let haystack = "Rust testing!";
    let start = 0;
    let end = haystack.len();
    let m = Match::new(haystack, start, end);
    let _ = core::fmt::format(format_args!("{:?}", m));
}

#[test]
fn test_match_debug_format_partial_string() {
    let haystack = "Boundary test.";
    let start = 1;
    let end = 10;
    let m = Match::new(haystack, start, end);
    let _ = core::fmt::format(format_args!("{:?}", m));
}

