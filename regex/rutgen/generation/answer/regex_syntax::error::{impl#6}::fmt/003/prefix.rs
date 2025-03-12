// Answer 0

#[test]
fn test_formatter_with_multiple_lines_and_error() {
    struct DummyError;
    impl core::fmt::Display for DummyError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "dummy error")
        }
    }
    
    let pattern = "abc\n(de|fg)\nxyz";
    let span_start = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 3 } };
    let span_end = ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 6 } };
    let spans = vec![span_start, span_end];

    let formatter = Formatter {
        pattern,
        err: &DummyError,
        span: &span_start,
        aux_span: Some(&span_end),
    };

    let mut buf = String::new();
    let _ = formatter.fmt(&mut buf);
}

#[test]
fn test_formatter_with_invalid_regex_error() {
    struct AnotherDummyError;
    impl core::fmt::Display for AnotherDummyError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "another dummy error")
        }
    }

    let pattern = "a*b+\n(?:[a-z]{3,}\n)";
    let span_start = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 5 } };
    let span_end = ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 11 } };
    let spans = vec![span_start, span_end];

    let formatter = Formatter {
        pattern,
        err: &AnotherDummyError,
        span: &span_start,
        aux_span: Some(&span_end),
    };

    let mut buf = String::new();
    let _ = formatter.fmt(&mut buf);
}

