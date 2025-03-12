// Answer 0

#[test]
fn test_prefix_matching_first_byte() {
    let haystack: &[u8] = &[b'a', b'b', b'c'];
    let span = Span { start: 0, end: 1 };
    let filter = Memchr2(b'a', b'b');
    let result = filter.prefix(haystack, span);
}

#[test]
fn test_prefix_matching_second_byte() {
    let haystack: &[u8] = &[b'a', b'b', b'c'];
    let span = Span { start: 1, end: 2 };
    let filter = Memchr2(b'b', b'a');
    let result = filter.prefix(haystack, span);
}

#[test]
fn test_prefix_with_non_matching_byte() {
    let haystack: &[u8] = &[b'a', b'b', b'c'];
    let span = Span { start: 1, end: 2 };
    let filter = Memchr2(b'c', b'a');
    let result = filter.prefix(haystack, span);
}

