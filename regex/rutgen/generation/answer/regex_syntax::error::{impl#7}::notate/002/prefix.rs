// Answer 0

#[test]
fn test_notate_with_multiple_lines_and_spans() {
    let pattern = "foo\nbar\nbaz";
    let line_number_width = 3;
    let by_line = vec![
        vec![ast::Span { start: Position::new(1, 1), end: Position::new(1, 4) }], // Spans for "foo"
        vec![ast::Span { start: Position::new(2, 1), end: Position::new(2, 4) }], // Spans for "bar"
        vec![ast::Span { start: Position::new(3, 1), end: Position::new(3, 4) }], // Spans for "baz"
    ];
    let multi_line: Vec<ast::Span> = vec![];

    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    let notated = spans.notate();
}

#[test]
fn test_notate_with_different_line_number_width() {
    let pattern = "hello\nworld\n!";
    let line_number_width = 4;
    let by_line = vec![
        vec![ast::Span { start: Position::new(1, 1), end: Position::new(1, 6) }], // Spans for "hello"
        vec![ast::Span { start: Position::new(2, 1), end: Position::new(2, 6) }], // Spans for "world"
        vec![], // No spans for "!"
    ];
    let multi_line: Vec<ast::Span> = vec![];

    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    let notated = spans.notate();
}

#[test]
fn test_notate_with_empty_lines() {
    let pattern = "first\n\nthird";
    let line_number_width = 2;
    let by_line = vec![
        vec![ast::Span { start: Position::new(1, 1), end: Position::new(1, 6) }], // Spans for "first"
        vec![], // No spans for empty line
        vec![ast::Span { start: Position::new(3, 1), end: Position::new(3, 6) }], // Spans for "third"
    ];
    let multi_line: Vec<ast::Span> = vec![];

    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    let notated = spans.notate();
}

#[test]
fn test_notate_with_one_line_and_multiple_spans() {
    let pattern = "example";
    let line_number_width = 5;
    let by_line = vec![
        vec![
            ast::Span { start: Position::new(1, 2), end: Position::new(1, 4) }, // Spans within "example"
            ast::Span { start: Position::new(1, 6), end: Position::new(1, 7) }, // Additional span
        ],
    ];
    let multi_line: Vec<ast::Span> = vec![];

    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    let notated = spans.notate();
}

