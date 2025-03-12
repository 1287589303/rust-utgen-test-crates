// Answer 0

#[test]
fn test_source_with_invalid_byte() {
    let error = DecodeError::InvalidByte(5, b'@');
    let decode_error = DecodeSliceError::DecodeError(error);
    let _result = decode_error.source();
}

#[test]
fn test_source_with_invalid_length() {
    let error = DecodeError::InvalidLength(3);
    let decode_error = DecodeSliceError::DecodeError(error);
    let _result = decode_error.source();
}

#[test]
fn test_source_with_invalid_last_symbol() {
    let error = DecodeError::InvalidLastSymbol(8, b'!');
    let decode_error = DecodeSliceError::DecodeError(error);
    let _result = decode_error.source();
}

#[test]
fn test_source_with_invalid_padding() {
    let error = DecodeError::InvalidPadding;
    let decode_error = DecodeSliceError::DecodeError(error);
    let _result = decode_error.source();
}

