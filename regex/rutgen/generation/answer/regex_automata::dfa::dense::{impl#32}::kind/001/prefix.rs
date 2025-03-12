// Answer 0

#[test]
fn test_kind_nfa() {
    let err = BuildError {
        kind: BuildErrorKind::NFA(nfa::thompson::BuildError {}),
    };
    let _result = err.kind();
}

#[test]
fn test_kind_word() {
    let err = BuildError {
        kind: BuildErrorKind::Word(look::UnicodeWordBoundaryError {}),
    };
    let _result = err.kind();
}

#[test]
fn test_kind_too_many_states() {
    let err = BuildError {
        kind: BuildErrorKind::TooManyStates { limit: 10 },
    };
    let _result = err.kind();
}

#[test]
fn test_kind_too_many_patterns() {
    let err = BuildError {
        kind: BuildErrorKind::TooManyPatterns { given: 20, limit: 15 },
    };
    let _result = err.kind();
}

#[test]
fn test_kind_exceeded_size_limit() {
    let err = BuildError {
        kind: BuildErrorKind::ExceededSizeLimit { limit: 1024 },
    };
    let _result = err.kind();
}

#[test]
fn test_kind_invalid_capture_index() {
    let err = BuildError {
        kind: BuildErrorKind::InvalidCaptureIndex { index: 5 },
    };
    let _result = err.kind();
}

#[test]
fn test_kind_unsupported() {
    let err = BuildError {
        kind: BuildErrorKind::Unsupported("unsupported feature"),
    };
    let _result = err.kind();
}

