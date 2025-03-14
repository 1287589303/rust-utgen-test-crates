// Answer 0

#[test]
fn test_decode_to_buf_valid_decoding() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3 // Example estimate: 4 base64 bytes decode to 3 raw bytes
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            if input == b"SGVsbG8=" {
                output.copy_from_slice(b"Hello");
                Ok(DecodeMetadata {
                    decoded_len: 5,
                    padding_offset: None,
                })
            } else {
                Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(0, input[0])))
            }
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let mut decoder_reader = DecoderReader::new(&b"SGVsbG8="[..], &engine);
    let mut output_buf = [0u8; 5];
    let result = decoder_reader.decode_to_buf(8, &mut output_buf);
    assert!(result.is_ok());
    assert_eq!(output_buf, b"Hello");
}

#[test]
#[should_panic]
fn test_decode_to_buf_too_small_buf() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {
                decoded_len: 5,
                padding_offset: None,
            })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let mut decoder_reader = DecoderReader::new(&b"SGVsbG8="[..], &engine);
    let mut output_buf = [0u8; 3]; // Smaller than needed
    decoder_reader.decode_to_buf(8, &mut output_buf);
}

#[test]
fn test_decode_to_buf_invalid_input() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(0, input[0])))
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let mut decoder_reader = DecoderReader::new(&b"INVALID"[..], &engine);
    let mut output_buf = [0u8; 5];
    let result = decoder_reader.decode_to_buf(7, &mut output_buf);
    assert!(result.is_err());
}

