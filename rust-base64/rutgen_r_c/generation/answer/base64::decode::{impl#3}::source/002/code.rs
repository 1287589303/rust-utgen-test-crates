// Answer 0

#[test]
fn test_source_decode_error() {
    struct TestDecodeError {
        offset: usize,
        byte: u8,
    }

    impl fmt::Debug for TestDecodeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "InvalidByte({}, {})", self.offset, self.byte)
        }
    }

    let error = DecodeError::InvalidByte(5, b'a');
    let decode_slice_error = DecodeSliceError::DecodeError(error);

    match decode_slice_error {
        DecodeSliceError::DecodeError(ref e) => {
            assert_eq!(decode_slice_error.source(), Some(e));
        },
        _ => panic!("Expected DecodeSliceError::DecodeError variant"),
    }
}

