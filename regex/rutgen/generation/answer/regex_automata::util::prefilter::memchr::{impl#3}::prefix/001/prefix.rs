// Answer 0

#[test]
fn test_prefix_haystack_empty() {
    let prefilter = Memchr2(1, 2);
    let haystack: &[u8] = &[];
    let span = Span { start: 0, end: 1 };
    let _ = prefilter.prefix(haystack, span);
}

#[test]
fn test_prefix_span_start_equal_haystack_length() {
    let prefilter = Memchr2(1, 2);
    let haystack: &[u8] = &[1, 2, 3];
    let span = Span { start: 3, end: 4 };
    let _ = prefilter.prefix(haystack, span);
}

#[test]
fn test_prefix_span_start_greater_than_haystack_length() {
    let prefilter = Memchr2(1, 2);
    let haystack: &[u8] = &[1, 2, 3];
    let span = Span { start: 4, end: 5 };
    let _ = prefilter.prefix(haystack, span);
}

