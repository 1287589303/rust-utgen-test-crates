// Answer 0

#[test]
fn test_prefix_haystack_empty() {
    let memchr = Memchr(b'a');
    let haystack: &[u8] = &[];
    let span = Span { start: 0, end: 1 };
    let _result = memchr.prefix(haystack, span);
}

#[test]
fn test_prefix_span_start_equal_to_haystack_length() {
    let memchr = Memchr(b'a');
    let haystack: &[u8] = b"abc";
    let span = Span { start: 3, end: 4 };
    let _result = memchr.prefix(haystack, span);
}

#[test]
fn test_prefix_span_start_greater_than_haystack_length() {
    let memchr = Memchr(b'a');
    let haystack: &[u8] = b"abc";
    let span = Span { start: 4, end: 5 };
    let _result = memchr.prefix(haystack, span);
}

