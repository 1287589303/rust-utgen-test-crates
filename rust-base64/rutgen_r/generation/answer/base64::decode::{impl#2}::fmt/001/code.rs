// Answer 0

#[test]
fn test_output_slice_too_small_fmt() {
    struct DecodeErrorWrapper;

    impl std::fmt::Debug for DecodeErrorWrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "mock error message")
        }
    }

    enum MyError {
        DecodeError(DecodeErrorWrapper),
        OutputSliceTooSmall,
    }

    let error = MyError::OutputSliceTooSmall;
    let mut output = String::new();
    let result = match error {
        MyError::DecodeError(_) => write!(&mut output, "DecodeError"),
        MyError::OutputSliceTooSmall => write!(&mut output, "Output slice too small"),
    };

    assert!(result.is_ok());
    assert_eq!(output, "Output slice too small");
}

