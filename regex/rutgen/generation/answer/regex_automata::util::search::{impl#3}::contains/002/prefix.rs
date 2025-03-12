// Answer 0

#[test]
fn test_contains_start_equals_offset_end_one() {
    let span = Span { start: 0, end: 1 };
    let result = span.contains(span.start);
}

#[test]
fn test_contains_start_equals_offset_end_two() {
    let span = Span { start: 1, end: 2 };
    let result = span.contains(span.start);
}

#[test]
fn test_contains_offset_in_range() {
    let span = Span { start: 0, end: 5 };
    let result = span.contains(span.start + 1);
}

#[test]
fn test_contains_offset_equals_end() {
    let span = Span { start: 3, end: 3 };
    let result = span.contains(span.start + 2);
}

#[test]
fn test_contains_large_range() {
    let span = Span { start: 100, end: 200 };
    let result = span.contains(span.start);
}

#[test]
fn test_contains_empty_span() {
    let span = Span { start: 200, end: 200 };
    let result = span.contains(span.start);
}

#[test]
fn test_contains_zero_offset_zero_end() {
    let span = Span { start: 0, end: 0 };
    let result = span.contains(0);
}

#[test]
fn test_contains_zero_offset_one_end() {
    let span = Span { start: 0, end: 1 };
    let result = span.contains(0);
}

#[test]
fn test_contains_10_offset_11_end() {
    let span = Span { start: 10, end: 11 };
    let result = span.contains(10);
}

#[test]
fn test_contains_15_offset_15_end() {
    let span = Span { start: 15, end: 15 };
    let result = span.contains(15);
}

