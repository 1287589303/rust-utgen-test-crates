// Answer 0

#[test]
fn test_formatter_with_multi_line_error() {
    struct TestError;
    impl core::fmt::Display for TestError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Test error display")
        }
    }
    
    let pattern = "first line\nsecond line\nthird line";
    let start_span = ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 10 } };
    let end_span = ast::Span { start: Position { line: 2, column: 0 }, end: Position { line: 2, column: 10 } };
    let multi_line_span = ast::Span { start: start_span.start, end: end_span.end };
    
    let span = ast::Span {
        start: Position { line: 1, column: 5 },
        end: Position { line: 2, column: 5 },
    };

    let spans = vec![multi_line_span.clone()];
    
    let formatter = Formatter {
        pattern,
        err: &TestError,
        span: &span,
        aux_span: None,
    };

    let mut buffer = core::fmt::Formatter::new();
    
    let result = formatter.fmt(&mut buffer);
}

