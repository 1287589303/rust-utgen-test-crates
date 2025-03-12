// Answer 0

#[test]
fn test_match_is_empty_empty_haystack() {
    let haystack: &[u8] = &[];
    let start = 0;
    let end = 0;
    let m = Match::new(haystack, start, end);
    m.is_empty();
}

#[test]
fn test_match_is_empty_zero_length_match() {
    let haystack: &[u8] = b"example";
    let start = 3;
    let end = 3;
    let m = Match::new(haystack, start, end);
    m.is_empty();
}

#[test]
fn test_match_is_empty_non_empty_match() {
    let haystack: &[u8] = b"example";
    let start = 2;
    let end = 5;
    let m = Match::new(haystack, start, end);
    m.is_empty();
}

#[test]
fn test_match_is_empty_boundary_start() {
    let haystack: &[u8] = b"";
    let start = 0;
    let end = 0;
    let m = Match::new(haystack, start, end);
    m.is_empty();
}

#[test]
fn test_match_is_empty_boundary_end() {
    let haystack: &[u8] = b"abc";
    let start = 1;
    let end = 1;
    let m = Match::new(haystack, start, end);
    m.is_empty();
}

#[test]
fn test_match_is_empty_multiple_cases() {
    let haystack: &[u8] = b"abc";
    let cases = [(0, 0), (1, 1), (0, 1), (2, 2)];

    for &(start, end) in &cases {
        let m = Match::new(haystack, start, end);
        m.is_empty();
    }
}

