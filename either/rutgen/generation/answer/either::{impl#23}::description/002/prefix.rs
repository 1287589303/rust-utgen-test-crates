// Answer 0

#[test]
fn test_description_left_simple_error() {
    struct SimpleError;
    impl fmt::Debug for SimpleError {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(fmt::Formatter::new(), "SimpleError")
        }
    }
    impl Error for SimpleError {
        fn description(&self) -> &str {
            "A simple error"
        }
    }

    let error_instance = Either::Left(SimpleError);
    let _ = error_instance.description();
}

#[test]
fn test_description_left_complex_error() {
    struct ComplexError;
    impl fmt::Debug for ComplexError {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(fmt::Formatter::new(), "ComplexError")
        }
    }
    impl Error for ComplexError {
        fn description(&self) -> &str {
            "A complex error"
        }
    }

    let error_instance = Either::Left(ComplexError);
    let _ = error_instance.description();
}

#[test]
fn test_description_right_simple_error() {
    struct SimpleError;
    impl fmt::Debug for SimpleError {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(fmt::Formatter::new(), "SimpleError")
        }
    }
    impl Error for SimpleError {
        fn description(&self) -> &str {
            "A simple error"
        }
    }

    let error_instance = Either::Right(SimpleError);
    let _ = error_instance.description();
}

#[test]
fn test_description_right_complex_error() {
    struct ComplexError;
    impl fmt::Debug for ComplexError {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(fmt::Formatter::new(), "ComplexError")
        }
    }
    impl Error for ComplexError {
        fn description(&self) -> &str {
            "A complex error"
        }
    }

    let error_instance = Either::Right(ComplexError);
    let _ = error_instance.description();
}

