// Answer 0

#[test]
fn test_is_end_crlf_case_1() {
    let mut matcher = LookMatcher::new();
    let haystack = b"test\n";
    let at = 1; // at is 1 and not end
    // haystack[at] == b'\n' && haystack[at - 1] != b'\r'
    assert!(matcher.is_end_crlf(haystack, at));
}

#[test]
fn test_is_end_crlf_case_2() {
    let mut matcher = LookMatcher::new();
    let haystack = b"test\r\n";
    let at = 5; // at is 5 and not end
    // haystack[at] == b'\n' && haystack[at - 1] != b'\r'
    assert!(matcher.is_end_crlf(haystack, at));
}

#[test]
fn test_is_end_crlf_case_3() {
    let mut matcher = LookMatcher::new();
    let haystack = b"test\r";
    let at = 4; // at is 4 and not end
    // haystack[at] == b'\r'
    assert!(matcher.is_end_crlf(haystack, at));
}

#[test]
fn test_is_end_crlf_case_4() {
    let mut matcher = LookMatcher::new();
    let haystack = b"\r\ntest";
    let at = 0; // at is 0 and not end
    // haystack[at] == b'\r'
    assert!(matcher.is_end_crlf(haystack, at));
}

