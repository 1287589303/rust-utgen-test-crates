// Answer 0

#[test]
fn test_kind_nfa() {
    let error = BuildError {
        kind: BuildErrorKind::NFA(nfa::thompson::BuildError::default()),
    };
    let _ = error.kind();
}

#[test]
fn test_kind_unsupported() {
    let error = BuildError {
        kind: BuildErrorKind::Unsupported("unsupported feature"),
    };
    let _ = error.kind();
}

#[test]
fn test_kind_too_many_states() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyStates {
            given: 100,
            limit: 50,
        },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_too_many_start_states() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyStartStates,
    };
    let _ = error.kind();
}

#[test]
fn test_kind_too_many_match_pattern_ids() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyMatchPatternIDs,
    };
    let _ = error.kind();
}

#[test]
fn test_kind_dfa_exceeded_size_limit() {
    let error = BuildError {
        kind: BuildErrorKind::DFAExceededSizeLimit { limit: 1024 },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_determinize_exceeded_size_limit() {
    let error = BuildError {
        kind: BuildErrorKind::DeterminizeExceededSizeLimit { limit: 2048 },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_insufficient_cache_capacity() {
    let error = BuildError {
        kind: BuildErrorKind::InsufficientCacheCapacity { minimum: 100, given: 50 },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_insufficient_state_id_capacity() {
    let error = BuildError {
        kind: BuildErrorKind::InsufficientStateIDCapacity { err: LazyStateIDError::default() },
    };
    let _ = error.kind();
}

#[cfg(feature = "syntax")]
#[test]
fn test_kind_syntax() {
    let error = BuildError {
        kind: BuildErrorKind::Syntax(regex_syntax::Error::default()),
    };
    let _ = error.kind();
}

#[test]
fn test_kind_captures() {
    let error = BuildError {
        kind: BuildErrorKind::Captures(captures::GroupInfoError::default()),
    };
    let _ = error.kind();
}

#[test]
fn test_kind_word() {
    let error = BuildError {
        kind: BuildErrorKind::Word(look::UnicodeWordBoundaryError::default()),
    };
    let _ = error.kind();
}

#[test]
fn test_kind_exceeded_size_limit() {
    let error = BuildError {
        kind: BuildErrorKind::ExceededSizeLimit { limit: 4096 },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_invalid_capture_index() {
    let error = BuildError {
        kind: BuildErrorKind::InvalidCaptureIndex { index: 99 },
    };
    let _ = error.kind();
}

#[cfg(feature = "syntax")]
#[test]
fn test_kind_unsupported_captures() {
    let error = BuildError {
        kind: BuildErrorKind::UnsupportedCaptures,
    };
    let _ = error.kind();
}

#[test]
fn test_kind_not_one_pass() {
    let error = BuildError {
        kind: BuildErrorKind::NotOnePass { msg: "not a one-pass regex" },
    };
    let _ = error.kind();
}

