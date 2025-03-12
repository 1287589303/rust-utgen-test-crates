// Answer 0

#[test]
fn test_is_end_crlf_at_valid_index_r() {
    let mut matcher = LookMatcher::new();
    let haystack: &[u8] = b"Hello\rWorld";
    let at: usize = 5; // Index of 'r', preconditions met
    matcher.is_end_crlf(haystack, at);
}

#[test]
fn test_is_end_crlf_at_valid_index_r2() {
    let mut matcher = LookMatcher::new();
    let haystack: &[u8] = b"Example\rText";
    let at: usize = 7; // Index of 'r', preconditions met
    matcher.is_end_crlf(haystack, at);
}

