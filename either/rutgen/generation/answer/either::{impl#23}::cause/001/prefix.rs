// Answer 0

#[test]
fn test_cause_with_right_variant_impl_error() {
    struct CustomError;

    impl fmt::Debug for CustomError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "CustomError")
        }
    }

    impl Error for CustomError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
        fn description(&self) -> &str {
            "A custom error"
        }
    }

    let right_variant = Either::Right(CustomError);
    let _result = right_variant.cause();
}

#[test]
fn test_cause_with_another_right_variant_impl_error() {
    struct AnotherCustomError;

    impl fmt::Debug for AnotherCustomError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "AnotherCustomError")
        }
    }

    impl Error for AnotherCustomError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
        fn description(&self) -> &str {
            "Another custom error"
        }
    }

    let right_variant = Either::Right(AnotherCustomError);
    let _result = right_variant.cause();
}

