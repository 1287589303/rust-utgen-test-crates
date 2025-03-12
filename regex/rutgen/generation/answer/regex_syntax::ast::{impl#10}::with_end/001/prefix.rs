// Answer 0

#[test]
fn test_with_end_updates_end_position() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let original_end_pos = Position { offset: 5, line: 1, column: 6 };
    let new_end_pos = Position { offset: 10, line: 1, column: 11 };
    
    let span = Span::new(start_pos, original_end_pos);
    let updated_span = span.with_end(new_end_pos);
}

#[test]
fn test_with_end_empty_span() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 0, line: 1, column: 1 };
    let new_end_pos = Position { offset: 1, line: 1, column: 2 };
    
    let span = Span::new(start_pos, end_pos);
    let updated_span = span.with_end(new_end_pos);
}

#[test]
fn test_with_end_single_position() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let new_end_pos = Position { offset: 1, line: 1, column: 2 };

    let span = Span::new(start_pos, start_pos);
    let updated_span = span.with_end(new_end_pos);
}

#[test]
fn test_with_end_zero_offset() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let new_end_pos = Position { offset: 0, line: 2, column: 1 };

    let span = Span::new(start_pos, start_pos);
    let updated_span = span.with_end(new_end_pos);
}

#[test]
fn test_with_end_boundary_line_column() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let new_end_pos = Position { offset: 5, line: 1, column: 1 };

    let span = Span::new(start_pos, start_pos);
    let updated_span = span.with_end(new_end_pos);
}

