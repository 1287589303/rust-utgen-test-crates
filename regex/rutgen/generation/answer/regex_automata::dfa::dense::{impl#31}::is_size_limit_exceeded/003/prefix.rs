// Answer 0

#[test]
fn test_is_size_limit_exceeded_too_many_match_pattern_ids() {
    let err = BuildError {
        kind: BuildErrorKind::TooManyMatchPatternIDs,
    };
    let result = err.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_too_many_states() {
    let err = BuildError {
        kind: BuildErrorKind::TooManyStates { given: 100, limit: 50 },
    };
    let result = err.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_too_many_start_states() {
    let err = BuildError {
        kind: BuildErrorKind::TooManyStartStates,
    };
    let result = err.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_dfa_exceeded_size_limit() {
    let err = BuildError {
        kind: BuildErrorKind::DFAExceededSizeLimit { limit: 1024 },
    };
    let result = err.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_determinize_exceeded_size_limit() {
    let err = BuildError {
        kind: BuildErrorKind::DeterminizeExceededSizeLimit { limit: 2048 },
    };
    let result = err.is_size_limit_exceeded();
}

