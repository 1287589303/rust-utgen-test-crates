// Answer 0

#[test]
fn test_encode_engine_slice_success() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let input_bytes = input.as_ref();
            let b64_encoded = base64::encode(input_bytes);
            let output_len = b64_encoded.len();
            if output_buf.len() < output_len {
                return Err(EncodeSliceError::OutputSliceTooSmall);
            }
            output_buf[..output_len].copy_from_slice(b64_encoded.as_bytes());
            Ok(output_len)
        }
    }

    let engine = MockEngine;
    let input = b"hello world";
    let mut output_buf = [0u8; 20]; // sufficiently large buffer
    let result = encode_engine_slice(input, &mut output_buf, &engine);
    assert_eq!(result, Ok(16)); // base64 of "hello world" is 16 bytes
    assert_eq!(&output_buf[..16], b"aGVsbG8gd29ybGQ=");
}

#[test]
fn test_encode_engine_slice_output_too_small() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let input_bytes = input.as_ref();
            let b64_encoded = base64::encode(input_bytes);
            let output_len = b64_encoded.len();
            if output_buf.len() < output_len {
                return Err(EncodeSliceError::OutputSliceTooSmall);
            }
            output_buf[..output_len].copy_from_slice(b64_encoded.as_bytes());
            Ok(output_len)
        }
    }

    let engine = MockEngine;
    let input = b"hello world";
    let mut output_buf = [0u8; 10]; // insufficient buffer
    let result = encode_engine_slice(input, &mut output_buf, &engine);
    assert_eq!(result, Err(EncodeSliceError::OutputSliceTooSmall));
}

