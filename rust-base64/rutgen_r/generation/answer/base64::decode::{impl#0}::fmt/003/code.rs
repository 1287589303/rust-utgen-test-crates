// Answer 0

#[test]
fn test_invalid_length_format() {
    struct InvalidLengthError {
        len: usize,
    }
    
    impl std::fmt::Display for InvalidLengthError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Invalid input length: {}", self.len)
        }
    }

    let error = InvalidLengthError { len: 5 };
    let result = format!("{}", error);
    assert_eq!(result, "Invalid input length: 5");
}

#[test]
fn test_invalid_length_format_zero() {
    struct InvalidLengthError {
        len: usize,
    }
    
    impl std::fmt::Display for InvalidLengthError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Invalid input length: {}", self.len)
        }
    }

    let error = InvalidLengthError { len: 0 };
    let result = format!("{}", error);
    assert_eq!(result, "Invalid input length: 0");
}

