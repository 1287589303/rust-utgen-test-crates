// Answer 0

#[test]
fn test_size_limit_with_nfa_error() {
    let error = BuildError {
        kind: BuildErrorKind::NFA(crate::nfa::thompson::BuildError),
    };
    error.size_limit();
}

#[test]
fn test_size_limit_with_word_error() {
    let error = BuildError {
        kind: BuildErrorKind::Word(look::UnicodeWordBoundaryError),
    };
    error.size_limit();
}

#[test]
fn test_size_limit_with_too_many_states() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyStates { limit: 10, given: 12 },
    };
    error.size_limit();
}

#[test]
fn test_size_limit_with_too_many_patterns() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyPatterns { given: 15, limit: 10 },
    };
    error.size_limit();
}

#[test]
fn test_size_limit_with_unsupported_look() {
    let error = BuildError {
        kind: BuildErrorKind::Unsupported("unsupported feature"),
    };
    error.size_limit();
}

#[test]
fn test_size_limit_with_invalid_capture_index() {
    let error = BuildError {
        kind: BuildErrorKind::InvalidCaptureIndex { index: u32::MAX },
    };
    error.size_limit();
}

