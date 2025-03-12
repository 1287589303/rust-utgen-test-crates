// Answer 0

#[test]
fn test_source_with_right_error() {
    struct TestError;
    
    impl fmt::Debug for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestError")
        }
    }

    impl Error for TestError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }

    let right_error = Either::Right(TestError);
    right_error.source();
}

#[test]
fn test_source_with_another_right_error() {
    struct AnotherError;

    impl fmt::Debug for AnotherError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AnotherError")
        }
    }

    impl Error for AnotherError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }

    let another_right_error = Either::Right(AnotherError);
    another_right_error.source();
}

#[test]
fn test_source_with_reference_error() {
    struct RefError;

    impl fmt::Debug for RefError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "RefError")
        }
    }

    impl Error for RefError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }

    let ref_error = Either::Right(RefError);
    ref_error.source();
}

