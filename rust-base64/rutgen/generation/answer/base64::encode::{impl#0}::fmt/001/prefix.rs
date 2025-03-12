// Answer 0

#[test]
fn test_encode_slice_error_display_output_slice_too_small() {
    let error = EncodeSliceError::OutputSliceTooSmall;
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", error);
}

#[test]
fn test_encode_slice_error_debug_output_slice_too_small() {
    let error = EncodeSliceError::OutputSliceTooSmall;
    let _ = format!("{:?}", error);
}

