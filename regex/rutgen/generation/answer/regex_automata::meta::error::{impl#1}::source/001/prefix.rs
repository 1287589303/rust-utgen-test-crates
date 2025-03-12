// Answer 0

#[test]
fn test_source_with_nfa_error() {
    struct FakeBuildError;
    impl std::error::Error for FakeBuildError {}

    let nfa_error = FakeBuildError;
    let build_error = BuildError {
        kind: BuildErrorKind::NFA(nfa_error),
    };

    let result = build_error.source();
}

#[test]
fn test_source_with_another_nfa_error() {
    struct AnotherFakeBuildError;
    impl std::error::Error for AnotherFakeBuildError {}

    let nfa_error = AnotherFakeBuildError;
    let build_error = BuildError {
        kind: BuildErrorKind::NFA(nfa_error),
    };

    let result = build_error.source();
}

