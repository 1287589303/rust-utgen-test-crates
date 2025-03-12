// Answer 0

#[test]
fn test_from_formatter_multiple_lines_with_aux_span() {
    struct DummyError;

    let pattern = "first line\nsecond line";
    let span = ast::Span { start: Position::new(0, 0), end: Position::new(0, 12) }; // Span covering first line
    let aux_span = Some(ast::Span { start: Position::new(1, 0), end: Position::new(1, 12) }); // Span covering second line

    let formatter = Formatter {
        pattern,
        err: &DummyError,
        span: &span,
        aux_span,
    };

    let spans = Spans::from_formatter(&formatter);
}

