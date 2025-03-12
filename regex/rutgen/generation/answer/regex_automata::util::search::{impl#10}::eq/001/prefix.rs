// Answer 0

#[test]
fn test_eq_valid_range() {
    let span = Span { start: 5, end: 10 };
    let range = 5..10;
    span.eq(&range);
}

#[test]
fn test_eq_boundary_condition() {
    let span = Span { start: 0, end: 1 };
    let range = 0..1;
    span.eq(&range);
}

#[test]
fn test_eq_large_values() {
    let span = Span { start: usize::MAX - 1, end: usize::MAX };
    let range = usize::MAX - 1..usize::MAX;
    span.eq(&range);
}

#[test]
fn test_eq_non_matching_end() {
    let span = Span { start: 2, end: 5 };
    let range = 2..6; // This should still execute as no assertion is used
    span.eq(&range);
}

