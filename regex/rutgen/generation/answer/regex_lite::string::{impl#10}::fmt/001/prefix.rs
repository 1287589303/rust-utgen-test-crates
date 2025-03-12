// Answer 0

#[test]
fn test_fmt_valid_match() {
    let haystack = "Hello, World!";
    let start = 1;
    let end = 5;
    
    let m = Match::new(haystack, start, end);
    let _ = format!("{:?}", m);
}

#[test]
fn test_fmt_empty_match() {
    let haystack = "Test String";
    let start = 5;
    let end = 5;

    let m = Match::new(haystack, start, end);
    let _ = format!("{:?}", m);
}

#[test]
fn test_fmt_match_with_boundary_start() {
    let haystack = "Boundary Test";
    let start = 0;
    let end = 8;

    let m = Match::new(haystack, start, end);
    let _ = format!("{:?}", m);
}

#[test]
fn test_fmt_match_with_boundary_end() {
    let haystack = "Test Example";
    let start = 4;
    let end = haystack.len();

    let m = Match::new(haystack, start, end);
    let _ = format!("{:?}", m);
}

#[test]
#[should_panic]
fn test_fmt_invalid_match_start_greater_than_end() {
    let haystack = "Invalid Match";
    let start = 7;
    let end = 3;

    let _ = Match::new(haystack, start, end);
}

#[test]
#[should_panic]
fn test_fmt_invalid_match_start_out_of_bounds() {
    let haystack = "Out of Bounds";
    let start = 15;
    let end = 16;

    let _ = Match::new(haystack, start, end);
}

#[test]
#[should_panic]
fn test_fmt_invalid_match_end_out_of_bounds() {
    let haystack = "Another Test";
    let start = 10;
    let end = 20;

    let _ = Match::new(haystack, start, end);
}

