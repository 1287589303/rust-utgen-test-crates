// Answer 0

#[test]
fn test_span_valid_range() {
    let haystack: &[u8] = b"hello";
    let input = Input::new(haystack).span(Span { start: 0, end: 5 });
}

#[test]
fn test_span_partial_range() {
    let haystack: &[u8] = b"world";
    let input = Input::new(haystack).span(Span { start: 1, end: 4 });
}

#[test]
fn test_span_valid_range_with_anchored() {
    let haystack: &[u8] = b"example";
    let input = Input::new(haystack).anchored(Anchored::Yes).span(Span { start: 0, end: 7 });
}

#[test]
fn test_span_with_earliest_true() {
    let haystack: &[u8] = b"finding";
    let input = Input::new(haystack).earliest(true).span(Span { start: 2, end: 5 });
}

#[test]
fn test_span_with_earliest_false() {
    let haystack: &[u8] = b"testcase";
    let input = Input::new(haystack).earliest(false).span(Span { start: 3, end: 7 });
}

#[test]
#[should_panic]
fn test_span_invalid_range_too_large() {
    let haystack: &[u8] = b"data";
    let input = Input::new(haystack).span(Span { start: 0, end: 5 });
}

#[test]
#[should_panic]
fn test_span_invalid_range_start_equals_end() {
    let haystack: &[u8] = b"check";
    let input = Input::new(haystack).span(Span { start: 2, end: 2 });
}

