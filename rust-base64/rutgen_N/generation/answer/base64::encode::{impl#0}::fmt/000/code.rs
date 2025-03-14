// Answer 0

#[test]
fn test_fmt_output_slice_too_small() {
    use std::fmt;

    struct EncoderError;

    impl fmt::Debug for EncoderError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                _ => write!(f, "EncoderError"),
            }
        }
    }

    impl fmt::Display for EncoderError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                EncoderError::OutputSliceTooSmall => write!(f, "Output slice too small"),
            }
        }
    }

    let err = EncoderError::OutputSliceTooSmall;
    let output = format!("{}", err);
    assert_eq!(output, "Output slice too small");
}

