// Answer 0

#[test]
fn test_is_empty_equal_offsets() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start, end);
    let _ = span.is_empty();
}

#[test]
fn test_is_empty_non_empty() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(start, end);
    let _ = span.is_empty();
}

#[test]
fn test_is_empty_non_empty_reverse() {
    let start = Position { offset: 1, line: 1, column: 2 };
    let end = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start, end);
    let _ = span.is_empty();
}

#[test]
fn test_is_empty_zero_offsets() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 0, line: 2, column: 1 };
    let span = Span::new(start, end);
    let _ = span.is_empty();
}

#[test]
fn test_is_empty_max_usize() {
    let start = Position { offset: usize::MAX, line: 1, column: 1 };
    let end = Position { offset: usize::MAX, line: 1, column: 1 };
    let span = Span::new(start, end);
    let _ = span.is_empty();
}

#[test]
fn test_is_empty_max_usize_non_empty() {
    let start = Position { offset: usize::MAX, line: 1, column: 1 };
    let end = Position { offset: usize::MAX - 1, line: 1, column: 2 };
    let span = Span::new(start, end);
    let _ = span.is_empty();
}

#[test]
fn test_is_empty_different_lines() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 0, line: 2, column: 1 };
    let span = Span::new(start, end);
    let _ = span.is_empty();
}

#[test]
fn test_is_empty_same_column() {
    let start = Position { offset: 5, line: 1, column: 5 };
    let end = Position { offset: 5, line: 1, column: 5 };
    let span = Span::new(start, end);
    let _ = span.is_empty();
}

