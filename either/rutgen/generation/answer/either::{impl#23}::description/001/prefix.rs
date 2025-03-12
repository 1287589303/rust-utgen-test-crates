// Answer 0

#[test]
fn test_description_right_variant_with_valid_error() {
    struct MockError;
    impl fmt::Debug for MockError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MockError")
        }
    }
    
    impl Error for MockError {
        fn source(&self) -> Option<&(dyn Error + 'static)> { None }
        fn description(&self) -> &str { "Mock error description" }
    }

    let error_instance = Either::Right(MockError);
    let _ = error_instance.description();
}

#[test]
fn test_description_right_variant_with_another_error() {
    struct AnotherMockError;
    impl fmt::Debug for AnotherMockError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AnotherMockError")
        }
    }
    
    impl Error for AnotherMockError {
        fn source(&self) -> Option<&(dyn Error + 'static)> { None }
        fn description(&self) -> &str { "Another mock error description" }
    }

    let error_instance = Either::Right(AnotherMockError);
    let _ = error_instance.description();
}

#[test]
fn test_description_right_variant_with_empty_error() {
    struct EmptyMockError;
    impl fmt::Debug for EmptyMockError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "EmptyMockError")
        }
    }
    
    impl Error for EmptyMockError {
        fn source(&self) -> Option<&(dyn Error + 'static)> { None }
        fn description(&self) -> &str { "Empty mock error" }
    }

    let error_instance = Either::Right(EmptyMockError);
    let _ = error_instance.description();
}

