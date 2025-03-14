// Answer 0

#[test]
fn test_source_decode_error() {
    struct TestDecodeError;
    
    impl fmt::Debug for TestDecodeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestDecodeError")
        }
    }
    
    impl error::Error for TestDecodeError {}
    
    let error_variant = DecodeSliceError::DecodeError(TestDecodeError);
    assert_eq!(error_variant.source(), Some(&TestDecodeError));
}

#[test]
fn test_source_output_slice_too_small() {
    let error_variant = DecodeSliceError::OutputSliceTooSmall;
    assert_eq!(error_variant.source(), None);
}

