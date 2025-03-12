// Answer 0

#[test]
fn test_is_start_crlf_start_at_zero() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"Hello\nWorld";
    let at = 0;
    matcher.is_start_crlf(haystack, at);
}

#[test]
fn test_is_start_crlf_newline_at_one() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"\nWorld";
    let at = 1;
    matcher.is_start_crlf(haystack, at);
}

#[test]
fn test_is_start_crlf_carriage_return_at_one() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"\rWorld";
    let at = 1;
    matcher.is_start_crlf(haystack, at);
}

#[test]
fn test_is_start_crlf_carriage_return_followed_by_not_newline() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"Hello\rWorld";
    let at = 7; // after 'Hello\r'
    matcher.is_start_crlf(haystack, at);
}

#[test]
fn test_is_start_crlf_carriage_return_followed_by_boundary_at_end() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"Hello\r"; // ending with a carriage return
    let at = 6; // at the end
    matcher.is_start_crlf(haystack, at);
}

