// Answer 0

#[test]
fn test_prefix_no_match_case1() {
    let haystack: &[u8] = &[4, 5, 6, 7, 8];
    let span = Span { start: 2, end: 3 };
    let filter = Memchr3(1, 2, 3);
    let _result = filter.prefix(haystack, span);
}

#[test]
fn test_prefix_no_match_case2() {
    let haystack: &[u8] = &[10, 11, 12, 13, 14];
    let span = Span { start: 1, end: 2 };
    let filter = Memchr3(5, 6, 7);
    let _result = filter.prefix(haystack, span);
}

#[test]
fn test_prefix_no_match_case3() {
    let haystack: &[u8] = &[20, 21, 22, 23];
    let span = Span { start: 0, end: 1 };
    let filter = Memchr3(30, 31, 32);
    let _result = filter.prefix(haystack, span);
}

