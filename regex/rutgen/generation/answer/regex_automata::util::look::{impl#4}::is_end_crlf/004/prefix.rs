// Answer 0

#[test]
fn test_is_end_crlf_with_r_at_1() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = &[b'a', b'\r', b'b', b'\n'];
    let at = 1;
    matcher.is_end_crlf(haystack, at);
}

#[test]
fn test_is_end_crlf_with_r_at_2() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = &[b'a', b'b', b'\r'];
    let at = 2;
    matcher.is_end_crlf(haystack, at);
}

#[test]
fn test_is_end_crlf_with_r_at_3() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = &[b'\n', b'\r', b'b'];
    let at = 3;
    matcher.is_end_crlf(haystack, at);
}

#[test]
fn test_is_end_crlf_with_r_at_4() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = &[b'\n', b'a', b'\r'];
    let at = 4;
    matcher.is_end_crlf(haystack, at);
}

