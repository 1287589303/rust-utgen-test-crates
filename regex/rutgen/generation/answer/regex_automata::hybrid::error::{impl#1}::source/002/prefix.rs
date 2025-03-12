// Answer 0

#[test]
fn test_source_nfa_build_error() {
    use crate::nfa::thompson::BuildError;

    let nfa_error = BuildError { /* initialize with valid parameters */ };
    let build_error = BuildError {
        kind: BuildErrorKind::NFA(nfa_error),
    };

    let _result = build_error.source();
}

#[test]
fn test_source_nfa_build_error_complete() {
    use crate::nfa::thompson::BuildError;

    let nfa_error = BuildError { /* initialize with valid parameters */ };
    let build_error = BuildError {
        kind: BuildErrorKind::NFA(nfa_error),
    };

    let _result = build_error.source();
}

#[test]
fn test_source_nfa_build_error_edge_case() {
    use crate::nfa::thompson::BuildError;

    let nfa_error = BuildError { /* initialize with edge case parameters */ };
    let build_error = BuildError {
        kind: BuildErrorKind::NFA(nfa_error),
    };

    let _result = build_error.source();
}

