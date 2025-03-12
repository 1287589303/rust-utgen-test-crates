// Answer 0

#[test]
fn test_source_with_left_variant_simple_error() {
    #[derive(Debug)]
    struct SimpleError;

    impl fmt::Display for SimpleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Simple error occurred")
        }
    }

    impl Error for SimpleError {}

    let left_error = Either::Left(SimpleError);
    let _ = left_error.source();
}

#[test]
fn test_source_with_left_variant_complex_error() {
    #[derive(Debug)]
    struct ComplexError;

    impl fmt::Display for ComplexError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Complex error occurred")
        }
    }

    impl Error for ComplexError {}

    let left_error = Either::Left(ComplexError);
    let _ = left_error.source();
}

#[test]
fn test_source_with_right_variant_simple_error() {
    #[derive(Debug)]
    struct SimpleError;

    impl fmt::Display for SimpleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Simple error occurred")
        }
    }

    impl Error for SimpleError {}

    let right_error = Either::Right(SimpleError);
    let _ = right_error.source();
}

#[test]
fn test_source_with_right_variant_complex_error() {
    #[derive(Debug)]
    struct ComplexError;

    impl fmt::Display for ComplexError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Complex error occurred")
        }
    }

    impl Error for ComplexError {}

    let right_error = Either::Right(ComplexError);
    let _ = right_error.source();
}

#[test]
fn test_source_with_both_variants() {
    #[derive(Debug)]
    struct SimpleError;

    impl fmt::Display for SimpleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Simple error occurred")
        }
    }

    impl Error for SimpleError {}

    #[derive(Debug)]
    struct ComplexError;

    impl fmt::Display for ComplexError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Complex error occurred")
        }
    }

    impl Error for ComplexError {}

    let both_errors = Either::Left(SimpleError);
    let _ = both_errors.source();

    let both_errors = Either::Right(ComplexError);
    let _ = both_errors.source();
}

