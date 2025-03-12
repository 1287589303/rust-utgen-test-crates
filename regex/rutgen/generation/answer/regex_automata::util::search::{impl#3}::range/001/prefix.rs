// Answer 0

#[test]
fn test_range_zero_to_zero() {
    let span = Span { start: 0, end: 0 };
    let _ = span.range();
}

#[test]
fn test_range_one_to_one() {
    let span = Span { start: 1, end: 1 };
    let _ = span.range();
}

#[test]
fn test_range_zero_to_ten() {
    let span = Span { start: 0, end: 10 };
    let _ = span.range();
}

#[test]
fn test_range_ten_to_ten() {
    let span = Span { start: 10, end: 10 };
    let _ = span.range();
}

#[test]
fn test_range_five_to_fifteen() {
    let span = Span { start: 5, end: 15 };
    let _ = span.range();
}

#[test]
fn test_range_usize_max_minus_one_to_usize_max() {
    let span = Span { start: usize::MAX - 1, end: usize::MAX };
    let _ = span.range();
}

