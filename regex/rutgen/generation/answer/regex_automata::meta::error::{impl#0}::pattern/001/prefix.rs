// Answer 0

#[test]
fn test_pattern_with_nfa_error() {
    let err = BuildError {
        kind: BuildErrorKind::NFA(nfa::thompson::BuildError::new(/* parameters */)),
    };
    err.pattern();
}

#[test]
fn test_pattern_with_unsupported_error() {
    let err = BuildError {
        kind: BuildErrorKind::Unsupported("unsupported feature"),
    };
    err.pattern();
}

#[test]
fn test_pattern_with_too_many_states_error() {
    let err = BuildError {
        kind: BuildErrorKind::TooManyStates { limit: 10 },
    };
    err.pattern();
}

#[test]
fn test_pattern_with_too_many_patterns_error() {
    let err = BuildError {
        kind: BuildErrorKind::TooManyPatterns { given: 5, limit: 4 },
    };
    err.pattern();
}

#[test]
fn test_pattern_with_exceeded_size_limit_error() {
    let err = BuildError {
        kind: BuildErrorKind::ExceededSizeLimit { limit: 1024 },
    };
    err.pattern();
}

#[test]
fn test_pattern_with_invalid_capture_index_error() {
    let err = BuildError {
        kind: BuildErrorKind::InvalidCaptureIndex { index: u32::MAX },
    };
    err.pattern();
}

