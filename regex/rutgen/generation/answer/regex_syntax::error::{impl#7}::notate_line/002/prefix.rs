// Answer 0

#[test]
fn test_notate_line_with_empty_span() {
    let pattern = "a.b.c";
    let line_number_width = 2;
    let spans = vec![vec![]];
    let multi_line = vec![];

    let spans_obj = Spans {
        pattern,
        line_number_width,
        by_line: spans,
        multi_line,
    };

    let result = spans_obj.notate_line(0);
    assert!(result.is_none());
}

#[test]
fn test_notate_line_with_single_span() {
    let pattern = "a.b.c";
    let line_number_width = 1;
    let spans = vec![vec![ast::Span { start: Position { column: 3 }, end: Position { column: 5 } }]];
    let multi_line = vec![];

    let spans_obj = Spans {
        pattern,
        line_number_width,
        by_line: spans,
        multi_line,
    };

    let result = spans_obj.notate_line(0);
    // The expected return value/type: Some(notes)
}

#[test]
fn test_notate_line_with_multiple_spans() {
    let pattern = "abc\nxyz";
    let line_number_width = 2;
    let spans = vec![
        vec![ast::Span { start: Position { column: 2 }, end: Position { column: 4 } }],
        vec![ast::Span { start: Position { column: 1 }, end: Position { column: 3 } }],
    ];
    let multi_line = vec![];

    let spans_obj = Spans {
        pattern,
        line_number_width,
        by_line: spans,
        multi_line,
    };

    let result_line_0 = spans_obj.notate_line(0);
    let result_line_1 = spans_obj.notate_line(1);
    // The expected return values/types: Some(notes) for both lines
}

#[test]
fn test_notate_line_with_no_padding() {
    let pattern = "abcd";
    let line_number_width = 0;
    let spans = vec![vec![ast::Span { start: Position { column: 1 }, end: Position { column: 3 } }]];
    let multi_line = vec![];

    let spans_obj = Spans {
        pattern,
        line_number_width,
        by_line: spans,
        multi_line,
    };

    let result = spans_obj.notate_line(0);
    // The expected return value/type: Some(notes)
}

#[test]
fn test_notate_line_with_padding_and_no_span() {
    let pattern = "abcde";
    let line_number_width = 3;
    let spans = vec![vec![]];
    let multi_line = vec![];

    let spans_obj = Spans {
        pattern,
        line_number_width,
        by_line: spans,
        multi_line,
    };

    let result = spans_obj.notate_line(0);
    assert!(result.is_none());
}

