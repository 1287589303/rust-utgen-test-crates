// Answer 0

#[cfg(feature = "dfa-build")]
#[test]
fn test_is_size_limit_exceeded_too_many_start_states() {
    let err = super::BuildError {
        kind: super::BuildErrorKind::TooManyStartStates,
    };
    let result = err.is_size_limit_exceeded();
}

#[cfg(feature = "dfa-build")]
#[test]
fn test_is_size_limit_exceeded_too_many_states() {
    let err = super::BuildError {
        kind: super::BuildErrorKind::TooManyStates { given: 100, limit: 50 },
    };
    let result = err.is_size_limit_exceeded();
}

#[cfg(feature = "dfa-build")]
#[test]
fn test_is_size_limit_exceeded_dfa_exceeded_size_limit() {
    let err = super::BuildError {
        kind: super::BuildErrorKind::DFAExceededSizeLimit { limit: 1024 },
    };
    let result = err.is_size_limit_exceeded();
}

#[cfg(feature = "dfa-build")]
#[test]
fn test_is_size_limit_exceeded_determinize_exceeded_size_limit() {
    let err = super::BuildError {
        kind: super::BuildErrorKind::DeterminizeExceededSizeLimit { limit: 2048 },
    };
    let result = err.is_size_limit_exceeded();
}

