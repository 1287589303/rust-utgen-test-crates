// Answer 0

#[derive(Debug, PartialEq)]
struct DecodeError;

#[derive(Debug, PartialEq)]
enum DecodeSliceError {
    DecodeError(DecodeError),
}

impl From<DecodeError> for DecodeSliceError {
    fn from(e: DecodeError) -> Self {
        DecodeSliceError::DecodeError(e)
    }
}

#[test]
fn test_from_decode_error() {
    let decode_error = DecodeError;
    let result = DecodeSliceError::from(decode_error);
    assert_eq!(result, DecodeSliceError::DecodeError(DecodeError));
}

#[test]
fn test_from_another_decode_error() {
    let decode_error = DecodeError;
    let result = DecodeSliceError::from(decode_error);
    assert!(matches!(result, DecodeSliceError::DecodeError(_)));
}

