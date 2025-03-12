// Answer 0

#[test]
fn test_source_valid_nfa_error() {
    use crate::nfa::thompson::BuildError;

    #[derive(Clone, Debug)]
    struct TestBuildError {
        error: BuildError,
    }

    impl TestBuildError {
        fn new() -> Self {
            TestBuildError {
                error: BuildError::NFA(BuildError::default()),
            }
        }
    }

    let test_error = TestBuildError::new();
    let result = test_error.error.kind();
    let source_result = if let BuildErrorKind::NFA(ref err) = result {
        Some(err)
    } else {
        None
    };

    let _ = source_result; // This line ensures the source result is acknowledged.
}

