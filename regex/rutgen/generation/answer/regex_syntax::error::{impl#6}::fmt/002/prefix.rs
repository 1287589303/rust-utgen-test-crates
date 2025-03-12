// Answer 0

#[test]
fn test_formatter_display_with_multiline_pattern() {
    struct ErrorImpl;
    impl core::fmt::Display for ErrorImpl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Sample error message")
        }
    }

    let pattern = "first line\nsecond line\nthird line";
    let span = &ast::Span {
        start: ast::Position { line: 1, column: 0 },
        end: ast::Position { line: 2, column: 5 },
    };
    let formatter = Formatter {
        pattern,
        err: &ErrorImpl,
        span,
        aux_span: None,
    };

    let _result = formatter.fmt(&mut core::fmt::Formatter::new());
}

#[test]
fn test_formatter_display_with_empty_aux_span() {
    struct ErrorImpl;
    impl core::fmt::Display for ErrorImpl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Another error message")
        }
    }

    let pattern = "line one\nline two\nline three";
    let span = &ast::Span {
        start: ast::Position { line: 2, column: 0 },
        end: ast::Position { line: 2, column: 10 },
    };
    let formatter = Formatter {
        pattern,
        err: &ErrorImpl,
        span,
        aux_span: None,
    };

    let _result = formatter.fmt(&mut core::fmt::Formatter::new());
}

#[test]
fn test_formatter_display_with_multiline_span() {
    struct ErrorImpl;
    impl core::fmt::Display for ErrorImpl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Another type of error")
        }
    }

    let pattern = "error on first line\nthis is a problem";
    let span = &ast::Span {
        start: ast::Position { line: 0, column: 0 },
        end: ast::Position { line: 1, column: 22 },
    };
    let formatter = Formatter {
        pattern,
        err: &ErrorImpl,
        span,
        aux_span: None,
    };

    let _result = formatter.fmt(&mut core::fmt::Formatter::new());
}

