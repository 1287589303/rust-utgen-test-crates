// Answer 0

#[test]
fn test_fmt_decode_error() {
    struct DecodeError(String);
    
    enum MyError {
        DecodeError(DecodeError),
        OutputSliceTooSmall,
    }

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::DecodeError(e) => write!(f, "DecodeError: {}", e.0),
                Self::OutputSliceTooSmall => write!(f, "Output slice too small"),
            }
        }
    }

    let error_instance = MyError::DecodeError(DecodeError("Invalid encoding".to_string()));
    let formatted = format!("{}", error_instance);
    assert_eq!(formatted, "DecodeError: Invalid encoding");
}

#[test]
fn test_fmt_output_slice_too_small() {
    enum MyError {
        DecodeError(String),
        OutputSliceTooSmall,
    }

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::DecodeError(e) => write!(f, "DecodeError: {}", e),
                Self::OutputSliceTooSmall => write!(f, "Output slice too small"),
            }
        }
    }

    let error_instance = MyError::OutputSliceTooSmall;
    let formatted = format!("{}", error_instance);
    assert_eq!(formatted, "Output slice too small");
}

