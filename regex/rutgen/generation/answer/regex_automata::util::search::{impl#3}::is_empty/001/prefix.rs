// Answer 0

#[test]
fn test_is_empty_with_start_equal_to_end() {
    let span = Span { start: 0, end: 0 };
    span.is_empty();
}

#[test]
fn test_is_empty_with_start_greater_than_end() {
    let span = Span { start: 1, end: 0 };
    span.is_empty();
}

#[test]
fn test_is_empty_with_start_less_than_end() {
    let span = Span { start: 2, end: 2 };
    span.is_empty();
}

#[test]
fn test_is_empty_with_start_equal_to_end_large() {
    let span = Span { start: 5, end: 5 };
    span.is_empty();
}

#[test]
fn test_is_empty_with_start_greater_than_end_large() {
    let span = Span { start: 10, end: 8 };
    span.is_empty();
}

