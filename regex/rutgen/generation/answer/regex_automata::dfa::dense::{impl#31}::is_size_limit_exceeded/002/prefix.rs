// Answer 0

#[test]
fn test_is_size_limit_exceeded_dfa_exceeded_size_limit_1() {
    let err = BuildError {
        kind: BuildErrorKind::DFAExceededSizeLimit { limit: 1 },
    };
    let result = err.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_dfa_exceeded_size_limit_100() {
    let err = BuildError {
        kind: BuildErrorKind::DFAExceededSizeLimit { limit: 100 },
    };
    let result = err.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_dfa_exceeded_size_limit_10000() {
    let err = BuildError {
        kind: BuildErrorKind::DFAExceededSizeLimit { limit: 10_000 },
    };
    let result = err.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_dfa_exceeded_size_limit_1000000() {
    let err = BuildError {
        kind: BuildErrorKind::DFAExceededSizeLimit { limit: 1_000_000 },
    };
    let result = err.is_size_limit_exceeded();
}

