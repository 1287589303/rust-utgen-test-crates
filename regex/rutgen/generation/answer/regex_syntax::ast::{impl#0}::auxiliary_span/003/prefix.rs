// Answer 0

#[test]
fn test_auxiliary_span_with_flag_repeated_negation() {
    let original_span = Span {
        start: Position(0),
        end: Position(5),
    };
    let error = Error {
        kind: ErrorKind::FlagRepeatedNegation { original: original_span },
        pattern: String::from("some pattern"),
        span: Span {
            start: Position(0),
            end: Position(15),
        },
    };
    let _ = error.auxiliary_span();
}

#[test]
fn test_auxiliary_span_with_flag_repeated_negation_boundary_case() {
    let original_span = Span {
        start: Position(0),
        end: Position(0),
    };
    let error = Error {
        kind: ErrorKind::FlagRepeatedNegation { original: original_span },
        pattern: String::from(""),
        span: Span {
            start: Position(0),
            end: Position(0),
        },
    };
    let _ = error.auxiliary_span();
}

