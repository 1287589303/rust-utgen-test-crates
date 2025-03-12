// Answer 0

#[test]
fn test_fmt_multiline_error() {
    struct MockError;

    impl core::fmt::Display for MockError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Mock error")
        }
    }

    let pattern = "This is the first line.\nThis is the second line.";
    let start_position = Position { line: 1, column: 5 };
    let end_position = Position { line: 2, column: 10 };

    let span = ast::Span {
        start: start_position,
        end: end_position,
    };

    let spans = Spans {
        pattern: pattern,
        line_number_width: 2,
        by_line: vec![vec![], vec![span.clone()]], 
        multi_line: vec![span],
    };
    
    let formatter = Formatter {
        pattern: pattern,
        err: &MockError,
        span: &ast::Span { start: start_position, end: end_position },
        aux_span: None,
    };

    let result = formatter.fmt(&mut core::fmt::Formatter::new());
}

