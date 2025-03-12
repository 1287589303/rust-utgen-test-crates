// Answer 0

#[test]
fn test_is_end_lf_non_end_condition() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(b'\n');
    let haystack: &[u8] = b"Hello World\n";
    let at = 5; // at index 5, haystack[5] is ' ' (space), which is not line terminator
    let result = matcher.is_end_lf(haystack, at);
}

#[test]
fn test_is_end_lf_line_terminator() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(b'\n');
    let haystack: &[u8] = b"Hello World\n";
    let at = 11; // at index 11, haystack[11] is '\n' (line terminator)
    let result = matcher.is_end_lf(haystack, at);
}

#[test]
fn test_is_end_lf_non_end_but_valid_index() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(b'\r');
    let haystack: &[u8] = b"Hello World\r";
    let at = 10; // at index 10, haystack[10] is 'd', which is not line terminator
    let result = matcher.is_end_lf(haystack, at);
}

