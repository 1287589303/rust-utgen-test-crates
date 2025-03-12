// Answer 0

#[test]
fn test_is_size_limit_exceeded_nfa_error() {
    let error = BuildError {
        kind: BuildErrorKind::NFA(nfa::thompson::BuildError::InvalidPattern),
    };
    let result = error.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_unsupported_error() {
    let error = BuildError {
        kind: BuildErrorKind::Unsupported("unsupported feature"),
    };
    let result = error.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_valid_nfa_error() {
    let error = BuildError {
        kind: BuildErrorKind::NFA(nfa::thompson::BuildError::TooManyStates {
            limit: 100,
        }),
    };
    let result = error.is_size_limit_exceeded();
}

