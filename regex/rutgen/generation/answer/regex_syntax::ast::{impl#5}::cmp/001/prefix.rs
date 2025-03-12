// Answer 0

#[test]
fn test_cmp_equal_spans() {
    let span1 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 5, line: 1, column: 6 },
    };
    let span2 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 5, line: 1, column: 6 },
    };
    span1.cmp(&span2);
}

#[test]
fn test_cmp_end_before_start() {
    let span1 = Span {
        start: Position { offset: 5, line: 1, column: 6 },
        end: Position { offset: 10, line: 1, column: 11 },
    };
    let span2 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 10, line: 1, column: 11 },
    };
    span1.cmp(&span2);
}

#[test]
fn test_cmp_different_lines() {
    let span1 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 5, line: 1, column: 6 },
    };
    let span2 = Span {
        start: Position { offset: 0, line: 2, column: 1 },
        end: Position { offset: 5, line: 2, column: 6 },
    };
    span1.cmp(&span2);
}

#[test]
fn test_cmp_offset_equal_line_different_column() {
    let span1 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 5, line: 1, column: 6 },
    };
    let span2 = Span {
        start: Position { offset: 0, line: 1, column: 2 },
        end: Position { offset: 5, line: 1, column: 7 },
    };
    span1.cmp(&span2);
}

#[test]
fn test_cmp_start_equal_end_different() {
    let span1 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 5, line: 1, column: 6 },
    };
    let span2 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 4, line: 1, column: 5 },
    };
    span1.cmp(&span2);
}

