// Answer 0

#[test]
fn test_is_end_lf_at_start() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(10); // LF
    let haystack: &[u8] = b"Hello\nWorld";
    let at = 0; // start of haystack
    matcher.is_end_lf(haystack, at);
}

#[test]
fn test_is_end_lf_at_end() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(10); // LF
    let haystack: &[u8] = b"Hello\nWorld";
    let at = haystack.len(); // end of haystack
    matcher.is_end_lf(haystack, at);
}

#[test]
fn test_is_end_lf_at_valid_line_terminator() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(10); // LF
    let haystack: &[u8] = b"Hello\nWorld";
    let at = 5; // position of 'o'
    matcher.is_end_lf(haystack, at);
} 

#[test]
fn test_is_end_lf_at_before_line_terminator() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(10); // LF
    let haystack: &[u8] = b"Hello\nWorld";
    let at = 6; // position of '\n'
    matcher.is_end_lf(haystack, at);
} 

#[test]
fn test_is_end_lf_at_after_line_terminator() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(10); // LF
    let haystack: &[u8] = b"Hello\nWorld";
    let at = 7; // position of 'W'
    matcher.is_end_lf(haystack, at);
} 

#[test]
fn test_is_end_lf_haystack_with_no_terminator() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(10); // LF
    let haystack: &[u8] = b"Hello World"; // no LF in haystack
    let at = 11; // at the end of haystack
    matcher.is_end_lf(haystack, at);
} 

