// Answer 0

#[test]
fn test_new_span_valid_positions() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 1, line: 1, column: 1 };
    let span = Span::new(start, end);
}

#[test]
fn test_new_span_same_start_end() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start, end);
}

#[test]
fn test_new_span_reversed_positions() {
    let start = Position { offset: 1, line: 1, column: 1 };
    let end = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start, end);
}

#[test]
fn test_new_span_large_positions() {
    let start = Position { offset: 10, line: 1, column: 11 };
    let end = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(start, end);
}

#[test]
fn test_new_span_maxed_out_positions() {
    let start = Position { offset: usize::MAX, line: usize::MAX, column: usize::MAX };
    let end = Position { offset: usize::MAX - 1, line: usize::MAX, column: usize::MAX };
    let span = Span::new(start, end);
}

