// Answer 0

#[test]
fn test_build_error_kind_syntax() {
    let error = BuildError {
        kind: BuildErrorKind::Syntax {
            pid: PatternID::new(1),
            err: regex_syntax::Error::new("syntax error"),
        },
    };
    let _result = error.source();
}

#[test]
fn test_build_error_kind_too_many_states() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyStates {
            limit: 5,
        },
    };
    let _result = error.source();
}

#[test]
fn test_build_error_kind_too_many_patterns() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyPatterns {
            limit: 10,
        },
    };
    let _result = error.source();
}

#[test]
fn test_build_error_kind_exceeded_size_limit() {
    let error = BuildError {
        kind: BuildErrorKind::ExceededSizeLimit {
            limit: 1024,
        },
    };
    let _result = error.source();
}

#[test]
fn test_build_error_kind_invalid_capture_index() {
    let error = BuildError {
        kind: BuildErrorKind::InvalidCaptureIndex {
            index: 100,
        },
    };
    let _result = error.source();
}

#[test]
fn test_build_error_kind_unsupported() {
    let error = BuildError {
        kind: BuildErrorKind::Unsupported("Unsupported feature"),
    };
    let _result = error.source();
}

#[test]
fn test_build_error_kind_word() {
    let error = BuildError {
        kind: BuildErrorKind::Word(look::UnicodeWordBoundaryError::new()),
    };
    let _result = error.source();
}

#[test]
fn test_build_error_kind_unsupported_captures() {
    let error = BuildError {
        kind: BuildErrorKind::UnsupportedCaptures,
    };
    let _result = error.source();
}

