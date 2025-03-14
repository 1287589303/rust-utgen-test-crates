// Answer 0

#[test]
fn test_fmt_decode_error() {
    struct MockDecodeError {
        offset: usize,
        byte: u8,
    }
    
    impl fmt::Display for MockDecodeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "InvalidByte({}, {})", self.offset, self.byte)
        }
    }
    
    let error_instance = DecodeError::InvalidByte(5, b'x');
    let decode_slice_error = DecodeSliceError::DecodeError(error_instance);
    
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| decode_slice_error.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "DecodeError: InvalidByte(5, 120)"); // 120 is the ASCII value for 'x'
}

#[test]
fn test_fmt_output_slice_too_small() {
    let decode_slice_error = DecodeSliceError::OutputSliceTooSmall;
    
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| decode_slice_error.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "Output slice too small");
}

