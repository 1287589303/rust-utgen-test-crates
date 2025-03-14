// Answer 0

#[test]
fn test_fmt_decode_error() {
    let error = DecodeSliceError::DecodeError(DecodeError::InvalidByte(5, b'x'));
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "DecodeError: InvalidByte(5, 'x')\n");
}

#[test]
fn test_fmt_output_slice_too_small() {
    let error = DecodeSliceError::OutputSliceTooSmall;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Output slice too small\n");
}

