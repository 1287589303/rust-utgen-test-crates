// Answer 0

#[test]
fn test_display_output_slice_too_small() {
    let error = DecodeSliceError::OutputSliceTooSmall;
    let result = format!("{}", error);
    assert_eq!(result, "Output slice too small");
}

