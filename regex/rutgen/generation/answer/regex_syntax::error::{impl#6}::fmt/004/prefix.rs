// Answer 0

#[test]
fn test_fmt_with_newline_patterns() {
    let pattern: &str = "abc\ndef\nghi";
    let err = "Syntax error";
    let start_span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 3 }};
    let end_span = ast::Span { start: Position { line: 2, column: 0 }, end: Position { line: 2, column: 3 }};
    let multi_line_span = vec![start_span, end_span];

    let span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 3, column: 0 }};
    let spans = Spans {
        pattern,
        line_number_width: 3,
        by_line: vec![vec![span]],
        multi_line: multi_line_span.clone(),
    };
    
    let formatter = Formatter {
        pattern,
        err,
        span: &span,
        aux_span: None,
    };

    let mut output = String::new();
    let result = formatter.fmt(&mut output);
}

#[test]
fn test_fmt_with_multiline_error_spans() {
    let pattern: &str = "first line\nsecond line\nthird line";
    let err = "An error occurred";
    
    let multi_line_span = vec![
        ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 10 }},
        ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 11 }},
    ];

    let span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 2, column: 0 }};
    
    let formatter = Formatter {
        pattern,
        err,
        span: &span,
        aux_span: None,
    };

    let mut output = String::new();
    let result = formatter.fmt(&mut output);
}

#[should_panic]
#[test]
fn test_fmt_with_invalid_output() {
    let pattern: &str = "line one\nline two";
    let err = "Error";

    let multi_line_span = vec![
        ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 5 }},
        ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 5 }},
    ];

    let span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 1, column: 0 }};
    
    let formatter = Formatter {
        pattern,
        err,
        span: &span,
        aux_span: None,
    };

    let mut output = String::new();
    let _result = formatter.fmt(&mut output);
}

