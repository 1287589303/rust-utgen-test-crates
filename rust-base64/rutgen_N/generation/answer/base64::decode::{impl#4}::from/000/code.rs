// Answer 0

#[derive(Debug)]
struct DecodeSliceError {
    error: DecodeError,
}

#[derive(Debug)]
struct DecodeError {
    message: String,
}

impl From<DecodeError> for DecodeSliceError {
    fn from(e: DecodeError) -> Self {
        DecodeSliceError::DecodeError(e)
    }
}

#[test]
fn test_from_decode_error() {
    let decode_error = DecodeError {
        message: String::from("Invalid base64 string"),
    };

    let decode_slice_error: DecodeSliceError = DecodeSliceError::from(decode_error);

    assert_eq!(decode_slice_error.error.message, "Invalid base64 string");
}

