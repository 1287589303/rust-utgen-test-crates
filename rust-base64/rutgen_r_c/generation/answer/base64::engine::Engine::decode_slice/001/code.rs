// Answer 0

#[test]
fn test_decode_slice_success() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
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
            &()
        }
    }

    let engine = TestEngine;
    let input = b"hello";
    let mut output = vec![0u8; 5];
    let result = engine.decode_slice(input, &mut output);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);
    assert_eq!(&output, b"hello");
}

#[test]
fn test_decode_slice_insufficient_output_buffer() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
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
            &()
        }
    }

    let engine = TestEngine;
    let input = b"hello";
    let mut output = [0u8; 3];
    let result = engine.decode_slice(input, &mut output);
    
    match result {
        Err(DecodeSliceError::OutputSliceTooSmall) => {},
        _ => panic!("Expected OutputSliceTooSmall error"),
    }
}

