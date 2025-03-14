// Answer 0

#[test]
fn test_source_output_slice_too_small() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct DecodeSliceErrorWrapper(DecodeSliceError);

    let error = DecodeSliceErrorWrapper(DecodeSliceError::OutputSliceTooSmall);
    assert_eq!(error.0.source(), None);
}

