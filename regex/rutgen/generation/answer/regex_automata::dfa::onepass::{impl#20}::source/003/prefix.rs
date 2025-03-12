// Answer 0

#[test]
fn test_source_nfa_error() {
    struct MockBuildError;
    impl std::error::Error for MockBuildError {}

    let nfa_error = MockBuildError;
    let build_error_kind = BuildErrorKind::NFA(nfa::thompson::BuildError);
    let build_error = BuildError { kind: build_error_kind };

    let result = build_error.source();
}

#[test]
fn test_source_word_error() {
    struct MockUnicodeWordBoundaryError;
    impl std::error::Error for MockUnicodeWordBoundaryError {}

    let word_error = MockUnicodeWordBoundaryError;
    let build_error_kind = BuildErrorKind::Word(UnicodeWordBoundaryError(()));
    let build_error = BuildError { kind: build_error_kind };

    let result = build_error.source();
}

