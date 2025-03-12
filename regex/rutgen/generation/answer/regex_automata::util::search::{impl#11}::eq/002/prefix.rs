// Answer 0

#[test]
fn test_eq_self_start_less_than_span_start() {
    let range = 5..10;
    let span = Span { start: 10, end: 15 };
    range.eq(&span);
}

#[test]
fn test_eq_self_start_greater_than_span_start() {
    let range = 15..20;
    let span = Span { start: 10, end: 15 };
    range.eq(&span);
}

#[test]
fn test_eq_self_start_zero_and_end_zero() {
    let range = 0..0;
    let span = Span { start: 1, end: 10 };
    range.eq(&span);
}

#[test]
fn test_eq_self_start_zero_and_span_end() {
    let range = 0..5;
    let span = Span { start: 5, end: 10 };
    range.eq(&span);
}

#[test]
fn test_eq_self_end_less_than_span_end() {
    let range = 5..8;
    let span = Span { start: 2, end: 10 };
    range.eq(&span);
}

#[test]
fn test_eq_self_start_and_end_overlap_span() {
    let range = 3..7;
    let span = Span { start: 10, end: 15 };
    range.eq(&span);
}

#[test]
fn test_eq_maximum_usize_values() {
    let range = usize::MAX - 5..usize::MAX;
    let span = Span { start: usize::MAX, end: usize::MAX };
    range.eq(&span);
}

