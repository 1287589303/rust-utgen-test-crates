// Answer 0

#[test]
fn test_eq_start_is_different() {
    let span = Span { start: 5, end: 10 };
    let range = Range { start: 6, end: 10 };
    span.eq(&range);
}

#[test]
fn test_eq_start_is_different_boundary() {
    let span = Span { start: 0, end: 1 };
    let range = Range { start: 1, end: 1 };
    span.eq(&range);
}

#[test]
fn test_eq_start_is_different_large_values() {
    let span = Span { start: usize::MAX, end: usize::MAX - 1 };
    let range = Range { start: usize::MAX - 1, end: usize::MAX - 1 };
    span.eq(&range);
}

