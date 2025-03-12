// Answer 0

#[test]
fn test_from_with_valid_match() {
    let haystack: &[u8] = b"hello world";
    let start = 0;
    let end = haystack.len();
    let m = Match::new(haystack, start, end);
    let range: core::ops::Range<usize> = m.into();
}

#[test]
fn test_from_with_empty_match() {
    let haystack: &[u8] = b"hello world";
    let start = 5;
    let end = 5;
    let m = Match::new(haystack, start, end);
    let range: core::ops::Range<usize> = m.into();
}

#[test]
fn test_from_with_partial_match() {
    let haystack: &[u8] = b"hello world";
    let start = 0;
    let end = 5;
    let m = Match::new(haystack, start, end);
    let range: core::ops::Range<usize> = m.into();
}

#[test]
fn test_from_with_full_haystack() {
    let haystack: &[u8] = b"hello world";
    let start = 0;
    let end = haystack.len();
    let m = Match::new(haystack, start, end);
    let range: core::ops::Range<usize> = m.into();
}

#[test]
fn test_from_with_substring_match() {
    let haystack: &[u8] = b"hello world";
    let start = 6;
    let end = 11;
    let m = Match::new(haystack, start, end);
    let range: core::ops::Range<usize> = m.into();
}

