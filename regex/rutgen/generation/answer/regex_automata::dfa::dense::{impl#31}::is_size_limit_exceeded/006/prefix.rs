// Answer 0

#[test]
fn test_is_size_limit_exceeded_unsupported() {
    let err = BuildError {
        kind: BuildErrorKind::Unsupported("Unsupported feature"),
    };
    let _result = err.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_nfa() {
    let err = BuildError {
        kind: BuildErrorKind::NFA(nfa::thompson::BuildError::default()),
    };
    let _result = err.is_size_limit_exceeded();
}

