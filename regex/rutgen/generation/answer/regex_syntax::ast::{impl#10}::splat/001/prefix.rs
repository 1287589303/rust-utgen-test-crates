// Answer 0

#[test]
fn test_splat_with_minimum_position() {
    let pos = Position { offset: 0, line: 1, column: 1 };
    let span = Span::splat(pos);
}

#[test]
fn test_splat_with_middle_position() {
    let pos = Position { offset: 500, line: 10, column: 5 };
    let span = Span::splat(pos);
}

#[test]
fn test_splat_with_maximum_offset() {
    let pos = Position { offset: 1000, line: 1, column: 2 };
    let span = Span::splat(pos);
}

#[test]
fn test_splat_with_high_line_and_column() {
    let pos = Position { offset: 250, line: 20, column: 30 };
    let span = Span::splat(pos);
}

