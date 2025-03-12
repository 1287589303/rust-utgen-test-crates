// Answer 0

#[test]
fn test_contains_non_empty_span_with_offset_in_range() {
    let span = Span { start: 1, end: 5 };
    let _ = span.contains(1);
    let _ = span.contains(2);
    let _ = span.contains(3);
    let _ = span.contains(4);
}

#[test]
fn test_contains_non_empty_span_with_offset_at_end() {
    let span = Span { start: 5, end: 5 };
    let _ = span.contains(5);
}

#[test]
fn test_contains_non_empty_span_with_offset_out_of_range() {
    let span = Span { start: 10, end: 15 };
    let _ = span.contains(16);
}

#[test]
fn test_contains_empty_span_with_offsets() {
    let span = Span { start: 0, end: 0 };
    let _ = span.contains(0);
    let _ = span.contains(1);
}

#[test]
fn test_contains_large_span_with_offset_before_start() {
    let span = Span { start: 100, end: 200 };
    let _ = span.contains(99);
}

#[test]
fn test_contains_large_span_with_offset_at_start() {
    let span = Span { start: 100, end: 200 };
    let _ = span.contains(100);
}

#[test]
fn test_contains_large_span_with_offset_at_end() {
    let span = Span { start: 100, end: 200 };
    let _ = span.contains(200);
}

#[test]
fn test_contains_large_span_with_offset_after_end() {
    let span = Span { start: 100, end: 200 };
    let _ = span.contains(201);
}

