// Answer 0

#[test]
fn test_source_with_unsupported_kind() {
    let build_error = BuildError {
        kind: BuildErrorKind::Unsupported("unsupported feature"),
    };
    let _ = build_error.source();
}

#[test]
fn test_source_with_too_many_states() {
    let build_error = BuildError {
        kind: BuildErrorKind::TooManyStates { limit: 10 },
    };
    let _ = build_error.source();
}

#[test]
fn test_source_with_too_many_patterns() {
    let build_error = BuildError {
        kind: BuildErrorKind::TooManyPatterns { limit: 20 },
    };
    let _ = build_error.source();
}

#[test]
fn test_source_with_exceeded_size_limit() {
    let build_error = BuildError {
        kind: BuildErrorKind::ExceededSizeLimit { limit: 1000 },
    };
    let _ = build_error.source();
}

#[test]
fn test_source_with_invalid_capture_index() {
    let build_error = BuildError {
        kind: BuildErrorKind::InvalidCaptureIndex { index: 42 },
    };
    let _ = build_error.source();
}

