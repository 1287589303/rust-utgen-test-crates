// Answer 0

#[test]
fn test_prefix_with_first_match() {
    let haystack: &[u8] = b"hello";
    let span = Span { start: 0, end: 1 };
    let prefilter = Memchr3(104, 101, 108); // 104 for 'h', 101 for 'e', 108 for 'l' 
    let result = prefilter.prefix(haystack, span);
}

#[test]
fn test_prefix_with_second_match() {
    let haystack: &[u8] = b"hello";
    let span = Span { start: 1, end: 2 };
    let prefilter = Memchr3(104, 101, 108); // 101 for 'e'
    let result = prefilter.prefix(haystack, span);
}

#[test]
fn test_prefix_with_third_match() {
    let haystack: &[u8] = b"hello";
    let span = Span { start: 2, end: 3 };
    let prefilter = Memchr3(104, 101, 108); // 108 for 'l'
    let result = prefilter.prefix(haystack, span);
}

#[test]
fn test_prefix_with_no_match() {
    let haystack: &[u8] = b"hello";
    let span = Span { start: 2, end: 3 };
    let prefilter = Memchr3(104, 101, 119); // 119 for 'w'
    let result = prefilter.prefix(haystack, span);
}

