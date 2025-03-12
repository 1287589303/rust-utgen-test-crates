// Answer 0

#[test]
fn test_len_empty_match() {
    let haystack: &[u8] = b"";
    let start = 0;
    let end = 0;
    let match_result = Match::new(haystack, start, end);
    let _ = match_result.len();
}

#[test]
fn test_len_single_byte_match() {
    let haystack: &[u8] = b"a";
    let start = 0;
    let end = 1;
    let match_result = Match::new(haystack, start, end);
    let _ = match_result.len();
}

#[test]
fn test_len_multiple_bytes_match() {
    let haystack: &[u8] = b"abcde";
    let start = 1;
    let end = 4;
    let match_result = Match::new(haystack, start, end);
    let _ = match_result.len();
}

#[test]
fn test_len_full_haystack() {
    let haystack: &[u8] = b"hello";
    let start = 0;
    let end = 5;
    let match_result = Match::new(haystack, start, end);
    let _ = match_result.len();
}

#[test]
fn test_len_boundary_conditions() {
    let haystack: &[u8] = b"boundary";
    let start = 0;
    let end = 8; // length of haystack
    let match_result = Match::new(haystack, start, end);
    let _ = match_result.len();

    let start = 8; // inclusive end
    let end = 8; // should be a valid case
    let match_result = Match::new(haystack, start, end);
    let _ = match_result.len();
}

#[test]
fn test_len_invalid_range() {
    let haystack: &[u8] = b"test";
    let start = 3; 
    let end = 2; // This creates an invalid range
    let match_result = Match::new(haystack, start, end);
    let _ = match_result.len();
}

