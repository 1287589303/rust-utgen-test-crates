// Answer 0

#[test]
fn test_source_with_word_error() {
    #[derive(Clone, Debug)]
    struct TestError(());
    
    let error_instance = UnicodeWordBoundaryError(TestError(()));
    let build_error = BuildError {
        kind: BuildErrorKind::Word(error_instance),
    };

    let _result = build_error.source();
}

#[test]
fn test_source_with_word_error_empty() {
    #[derive(Clone, Debug)]
    struct TestError(());
    
    let error_instance = UnicodeWordBoundaryError(TestError(()));
    let build_error = BuildError {
        kind: BuildErrorKind::Word(error_instance),
    };

    let _result = build_error.source();
}

