// Answer 0

#[test]
fn test_fmt_single_line_no_newline() {
    let pattern = "abc";
    let err = "some error";
    let span = ast::Span {
        start: Position { line: 0, column: 0 },
        end: Position { line: 0, column: 3 },
    };
    let formatter = Formatter {
        pattern,
        err: &err,
        span: &span,
        aux_span: None,
    };

    let mut output = Vec::new();
    {
        let f = &mut core::fmt::Formatter::from_writer(&mut output);
        let _ = formatter.fmt(f);
    }
}

#[test]
fn test_fmt_single_line_err_notated() {
    let pattern = "abc";
    let err = "some error";
    let span = ast::Span {
        start: Position { line: 0, column: 0 },
        end: Position { line: 0, column: 3 },
    };
    let formatter = Formatter {
        pattern,
        err: &err,
        span: &span,
        aux_span: None,
    };

    let mut output = Vec::new();
    {
        let f = &mut core::fmt::Formatter::from_writer(&mut output);
        let result = formatter.fmt(f);
        // Expect that the result is Ok
        assert!(result.is_ok());
    }
}

