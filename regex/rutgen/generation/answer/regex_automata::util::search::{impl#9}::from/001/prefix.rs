// Answer 0

#[test]
fn test_from_span_zero() {
    let span = Span { start: 0, end: 0 };
    let range: Range<usize> = Range::from(span);
}

#[test]
fn test_from_span_positive_range() {
    let span = Span { start: 5, end: 10 };
    let range: Range<usize> = Range::from(span);
}

#[test]
fn test_from_span_edge_case_max() {
    let span = Span { start: usize::MAX - 1, end: usize::MAX };
    let range: Range<usize> = Range::from(span);
}

#[test]
#[should_panic]
fn test_from_span_invalid_case() {
    let span = Span { start: 10, end: 5 };
    let range: Range<usize> = Range::from(span);
}

