// Answer 0

#[test]
fn test_partial_cmp_equal_positions() {
    let span1 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 0, line: 1, column: 1 },
    };
    let span2 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 0, line: 1, column: 1 },
    };
    span1.partial_cmp(&span2);
}

#[test]
fn test_partial_cmp_different_end_positions() {
    let span1 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 1, line: 1, column: 2 },
    };
    let span2 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 0, line: 1, column: 1 },
    };
    span1.partial_cmp(&span2);
}

#[test]
fn test_partial_cmp_reverse_order() {
    let span1 = Span {
        start: Position { offset: 1, line: 1, column: 2 },
        end: Position { offset: 0, line: 1, column: 1 },
    };
    let span2 = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 1, line: 1, column: 2 },
    };
    span1.partial_cmp(&span2);
}

#[test]
fn test_partial_cmp_same_start_end() {
    let span1 = Span {
        start: Position { offset: 1, line: 1, column: 2 },
        end: Position { offset: 1, line: 1, column: 2 },
    };
    let span2 = Span {
        start: Position { offset: 1, line: 1, column: 2 },
        end: Position { offset: 1, line: 1, column: 2 },
    };
    span1.partial_cmp(&span2);
}

