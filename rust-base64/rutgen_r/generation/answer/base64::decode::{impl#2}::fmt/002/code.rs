// Answer 0

#[test]
fn test_fmt_decode_error() {
    struct DecodeError(String);
    
    enum Error {
        DecodeError(DecodeError),
        OutputSliceTooSmall,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::DecodeError(e) => write!(f, "DecodeError: {}", e.0),
                Self::OutputSliceTooSmall => write!(f, "Output slice too small"),
            }
        }
    }

    let error_message = "Invalid character found";
    let error = Error::DecodeError(DecodeError(error_message.to_string()));
    
    let result = format!("{}", error);
    assert_eq!(result, "DecodeError: Invalid character found");
}

#[test]
fn test_fmt_output_slice_too_small() {
    struct DecodeError(String);
    
    enum Error {
        DecodeError(DecodeError),
        OutputSliceTooSmall,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::DecodeError(e) => write!(f, "DecodeError: {}", e.0),
                Self::OutputSliceTooSmall => write!(f, "Output slice too small"),
            }
        }
    }

    let error = Error::OutputSliceTooSmall;
    
    let result = format!("{}", error);
    assert_eq!(result, "Output slice too small");
}

