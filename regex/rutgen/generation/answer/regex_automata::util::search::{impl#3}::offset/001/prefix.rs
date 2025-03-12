// Answer 0

#[test]
fn test_offset_zero() {
    let span = Span { start: 5, end: 7 };
    let offset = 0;
    let result = span.offset(offset);
}

#[test]
fn test_offset_positive() {
    let span = Span { start: 3, end: 8 };
    let offset = 2;
    let result = span.offset(offset);
}

#[test]
fn test_offset_boundary() {
    let span = Span { start: 0, end: 10 };
    let offset = 5;
    let result = span.offset(offset);
}

#[test]
fn test_offset_maximum() {
    let span = Span { start: 1, end: 2 };
    let offset = 10;
    let result = span.offset(offset);
}

#[test]
fn test_offset_minimum() {
    let span = Span { start: 9, end: 10 };
    let offset = 0;
    let result = span.offset(offset);
}

#[test]
fn test_offset_overlap() {
    let span = Span { start: 2, end: 6 };
    let offset = 4;
    let result = span.offset(offset);
}

