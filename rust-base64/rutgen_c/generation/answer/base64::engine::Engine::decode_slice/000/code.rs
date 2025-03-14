// Answer 0

#[test]
fn test_decode_slice_success() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0 // Dummy implementation
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len // Assume 1:1 for simplicity
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            if output.len() < decode_estimate {
                return Err(DecodeSliceError::OutputSliceTooSmall);
            }
            output[..decode_estimate].copy_from_slice(input);
            Ok(DecodeMetadata { decoded_len: decode_estimate })
        }
        
        fn config(&self) -> &Self::Config {
            &() // Dummy implementation
        }
    }

    let engine = TestEngine;
    let input = b"test";
    let mut output = vec![0u8; 4];
    
    let result = engine.decode_slice(input, &mut output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 4);
    assert_eq!(output, b"test");
}

#[test]
fn test_decode_slice_output_too_small() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0 // Dummy implementation
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len // Assume 1:1 for simplicity
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            if output.len() < decode_estimate {
                return Err(DecodeSliceError::OutputSliceTooSmall);
            }
            output[..decode_estimate].copy_from_slice(input);
            Ok(DecodeMetadata { decoded_len: decode_estimate })
        }
        
        fn config(&self) -> &Self::Config {
            &() // Dummy implementation
        }
    }

    let engine = TestEngine;
    let input = b"test";
    let mut output = vec![0u8; 2]; // Smaller than needed

    let result = engine.decode_slice(input, &mut output);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DecodeSliceError::OutputSliceTooSmall);
}

