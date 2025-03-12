// Answer 0

#[test]
fn test_get_range_full_haystack() {
    let haystack: &[u8] = b"hello";
    let span = Span { start: 0, end: 5 };
    let input = Input::new(haystack).span(span);
    input.get_range();
}

#[test]
fn test_get_range_partial_haystack() {
    let haystack: &[u8] = b"world";
    let span = Span { start: 1, end: 4 };
    let input = Input::new(haystack).span(span);
    input.get_range();
}

#[test]
fn test_get_range_empty_span() {
    let haystack: &[u8] = b"foo";
    let span = Span { start: 1, end: 1 };
    let input = Input::new(haystack).span(span);
    input.get_range();
}

#[test]
fn test_get_range_span_at_end() {
    let haystack: &[u8] = b"test";
    let span = Span { start: 3, end: 4 };
    let input = Input::new(haystack).span(span);
    input.get_range();
}

#[test]
fn test_get_range_span_exceeds_haystack() {
    let haystack: &[u8] = b"example";
    let span = Span { start: 0, end: 8 };
    let input = Input::new(haystack).span(span);
    input.get_range();
}

#[test]
fn test_get_range_with_anchored_no() {
    let haystack: &[u8] = b"test";
    let span = Span { start: 0, end: 4 };
    let input = Input::new(haystack).span(span).anchored(Anchored::No);
    input.get_range();
}

#[test]
fn test_get_range_with_anchored_yes() {
    let haystack: &[u8] = b"example";
    let span = Span { start: 0, end: 7 };
    let input = Input::new(haystack).span(span).anchored(Anchored::Yes);
    input.get_range();
}

#[test]
fn test_get_range_with_earliest_true() {
    let haystack: &[u8] = b"string";
    let span = Span { start: 0, end: 6 };
    let input = Input::new(haystack).span(span).earliest(true);
    input.get_range();
}

#[test]
fn test_get_range_with_earliest_false() {
    let haystack: &[u8] = b"data";
    let span = Span { start: 0, end: 4 };
    let input = Input::new(haystack).span(span).earliest(false);
    input.get_range();
}

