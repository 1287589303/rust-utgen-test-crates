// Answer 0

#[test]
fn test_auxiliary_span_no_auxiliary_span_for_unicode_not_allowed() {
    let error = Error {
        kind: ErrorKind::UnicodeNotAllowed,
        pattern: String::from("(?-u:\\pL)"),
        span: Span { start: Position(0), end: Position(10) },
    };
    let _ = error.auxiliary_span();
}

#[test]
fn test_auxiliary_span_no_auxiliary_span_for_invalid_utf8() {
    let error = Error {
        kind: ErrorKind::InvalidUtf8,
        pattern: String::from("invalid utf8"),
        span: Span { start: Position(0), end: Position(14) },
    };
    let _ = error.auxiliary_span();
}

#[test]
fn test_auxiliary_span_no_auxiliary_span_for_invalid_line_terminator() {
    let error = Error {
        kind: ErrorKind::InvalidLineTerminator,
        pattern: String::from("invalid line terminator"),
        span: Span { start: Position(0), end: Position(25) },
    };
    let _ = error.auxiliary_span();
}

