// Answer 0

#[test]
fn test_source_with_unsupported() {
    let error = BuildError {
        kind: BuildErrorKind::Unsupported("unsupported feature"),
    };
    let _result = error.source();
}

#[test]
fn test_source_with_too_many_states() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyStates { limit: 10 },
    };
    let _result = error.source();
}

#[test]
fn test_source_with_too_many_patterns() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyPatterns { limit: 5 },
    };
    let _result = error.source();
}

#[test]
fn test_source_with_dfa_exceeded_size_limit() {
    let error = BuildError {
        kind: BuildErrorKind::DFAExceededSizeLimit { limit: 1024 },
    };
    let _result = error.source();
}

#[test]
fn test_source_with_determinize_exceeded_size_limit() {
    let error = BuildError {
        kind: BuildErrorKind::DeterminizeExceededSizeLimit { limit: 2048 },
    };
    let _result = error.source();
}

#[test]
fn test_source_with_insufficient_cache_capacity() {
    let error = BuildError {
        kind: BuildErrorKind::InsufficientCacheCapacity { minimum: 20, given: 10 },
    };
    let _result = error.source();
}

#[test]
fn test_source_with_insufficient_state_id_capacity() {
    let error = BuildError {
        kind: BuildErrorKind::InsufficientStateIDCapacity { err: LazyStateIDError },
    };
    let _result = error.source();
}

