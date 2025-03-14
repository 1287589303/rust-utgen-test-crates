// Answer 0

#[derive(Debug)]
enum DecodeSliceError {
    DecodeError(Box<dyn std::error::Error + 'static>),
    OutputSliceTooSmall,
}

impl DecodeSliceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DecodeSliceError::DecodeError(e) => Some(e),
            DecodeSliceError::OutputSliceTooSmall => None,
        }
    }
}

#[test]
fn test_source_with_decode_error() {
    struct TestError;

    impl std::fmt::Debug for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "TestError")
        }
    }

    impl std::error::Error for TestError {}

    let error: Box<dyn std::error::Error> = Box::new(TestError);
    let decode_error = DecodeSliceError::DecodeError(error);
    let result = decode_error.source();

    assert!(result.is_some());
}

