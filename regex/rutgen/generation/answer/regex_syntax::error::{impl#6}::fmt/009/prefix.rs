// Answer 0

#[test]
fn test_formatter_with_multiline_error() {
    let pattern = "a(bc\nd)"; // Multi-line pattern
    let error_instance = "Duplicate capture group"; // Error implements fmt::Display
    let line_number_width = 4; // Non-zero value
    let span_start = ast::Span { start: Position { column: 0, line: 0 }, end: Position { column: 2, line: 0 } };
    let span_end = ast::Span { start: Position { column: 0, line: 1 }, end: Position { column: 1, line: 1 } };
    
    let multi_line_spans = vec![
        span_start,
        span_end,
    ];
    
    let span = ast::Span {
        start: Position { column: 0, line: 0 },
        end: Position { column: 3, line: 1 },
    };

    let span_vec = vec![span]; // Prepare spans to be empty
    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line: vec![vec![span_start], vec![span_end]],
        multi_line: span_vec,
    };
    
    let formatter = Formatter {
        pattern,
        err: &error_instance,
        span: &span_start,
        aux_span: None,
    };
    
    let _ = formatter.fmt(&mut core::fmt::Formatter::new());
}

#[test]
fn test_formatter_with_single_line_error() {
    let pattern = "abcd"; // Single line pattern
    let error_instance = "Invalid character"; // Error implements fmt::Display
    let line_number_width = 5; // Non-zero value
    
    let span = ast::Span {
        start: Position { column: 0, line: 0 },
        end: Position { column: 3, line: 0 },
    };

    let multi_line_spans = vec![]; // No multi-line spans
    
    let but_span = vec![span];
    
    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line: vec![vec![span]],
        multi_line: multi_line_spans,
    };
    
    let formatter = Formatter {
        pattern,
        err: &error_instance,
        span: &span,
        aux_span: None,
    };

    let _ = formatter.fmt(&mut core::fmt::Formatter::new());
}

