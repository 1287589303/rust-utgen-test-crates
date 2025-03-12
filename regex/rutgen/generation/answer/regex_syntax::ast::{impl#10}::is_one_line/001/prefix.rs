// Answer 0

#[test]
fn test_is_one_line_true_same_line() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(start_position, end_position);
    let result = span.is_one_line();
}

#[test]
fn test_is_one_line_true_another_same_line() {
    let start_position = Position { offset: 10, line: 2, column: 1 };
    let end_position = Position { offset: 15, line: 2, column: 6 };
    let span = Span::new(start_position, end_position);
    let result = span.is_one_line();
}

#[test]
fn test_is_one_line_false_different_lines() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 1, line: 2, column: 1 };
    let span = Span::new(start_position, end_position);
    let result = span.is_one_line();
}

#[test]
fn test_is_one_line_false_another_different_lines() {
    let start_position = Position { offset: 0, line: 3, column: 1 };
    let end_position = Position { offset: 4, line: 4, column: 1 };
    let span = Span::new(start_position, end_position);
    let result = span.is_one_line();
}

