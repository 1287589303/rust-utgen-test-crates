// Answer 0

#[test]
fn test_contains_empty_span_with_offset_less_than_start() {
    let span = Span { start: 0, end: 0 };
    let offset = 0;
    span.contains(offset);
}

#[test]
fn test_contains_empty_span_with_offset_equal_to_start() {
    let span = Span { start: 0, end: 0 };
    let offset = 0;
    span.contains(offset);
}

#[test]
fn test_contains_empty_span_with_offset_equal_to_end() {
    let span = Span { start: 0, end: 0 };
    let offset = 0;
    span.contains(offset);
}

#[test]
fn test_contains_empty_span_with_offset_greater_than_end() {
    let span = Span { start: 0, end: 0 };
    let offset = 1;
    span.contains(offset);
}

#[test]
fn test_contains_empty_span_with_negative_offset() {
    let span = Span { start: 0, end: 0 };
    let offset = usize::MAX;
    span.contains(offset);
}

