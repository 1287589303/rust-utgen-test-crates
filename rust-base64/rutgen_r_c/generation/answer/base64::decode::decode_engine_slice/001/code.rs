// Answer 0

#[test]
fn test_decode_engine_slice_success() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn decode_slice<T: AsRef<[u8]>>(
            &self,
            _input: T,
            output: &mut [u8],
        ) -> Result<usize, DecodeSliceError> {
            let data = b"Hello, world!";
            if output.len() < data.len() {
                return Err(DecodeSliceError::OutputSliceTooSmall);
            }
            output.copy_from_slice(data);
            Ok(data.len())
        }
    }

    let input = "SGVsbG8sIHdvcmxkIQ=="; // base64 for "Hello, world!"
    let mut output = vec![0u8; 14]; // buffer larger than needed
    let engine = MockEngine;

    let result = decode_engine_slice(input, &mut output, &engine);
    assert_eq!(result, Ok(14));
    assert_eq!(&output[..14], b"Hello, world!");
}

#[test]
fn test_decode_engine_slice_output_too_small() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn decode_slice<T: AsRef<[u8]>>(
            &self,
            _input: T,
            output: &mut [u8],
        ) -> Result<usize, DecodeSliceError> {
            let data = b"Hello, world!";
            if output.len() < data.len() {
                return Err(DecodeSliceError::OutputSliceTooSmall);
            }
            output.copy_from_slice(data);
            Ok(data.len())
        }
    }

    let input = "SGVsbG8sIHdvcmxkIQ=="; // base64 for "Hello, world!"
    let mut output = vec![0u8; 5]; // buffer too small
    let engine = MockEngine;

    let result = decode_engine_slice(input, &mut output, &engine);
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
}

