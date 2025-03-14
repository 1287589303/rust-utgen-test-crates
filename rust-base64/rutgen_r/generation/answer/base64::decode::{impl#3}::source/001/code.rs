// Answer 0

#[test]
fn test_source_output_slice_too_small() {
    struct DecodeSliceError {
        kind: DecodeSliceErrorKind,
    }

    enum DecodeSliceErrorKind {
        DecodeError(Box<dyn std::error::Error>),
        OutputSliceTooSmall,
    }

    impl DecodeSliceError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self.kind {
                DecodeSliceErrorKind::DecodeError(ref e) => Some(e.as_ref()),
                DecodeSliceErrorKind::OutputSliceTooSmall => None,
            }
        }
    }

    let error_instance = DecodeSliceError {
        kind: DecodeSliceErrorKind::OutputSliceTooSmall,
    };

    assert_eq!(error_instance.source(), None);
}

