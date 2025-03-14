// Answer 0

#[test]
fn test_read_with_full_buffer() {
    struct MockEngine;
    
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3 // Base64 decoding estimate
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            let b64_len = input.len();
            output[..b64_len / 4 * 3].copy_from_slice(&input[0..(b64_len / 4 * 3)]);
            Ok(DecodeMetadata { decoded_len: b64_len / 4 * 3, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let input_data = "QUJD"; // Base64 for "ABC"
    let mut reader = std::io::Cursor::new(input_data.as_bytes());
    let mut decoder_reader = DecoderReader::new(reader, &engine);
    
    let mut buf = [0u8; 3]; // Buffer for decoded output
    let result = decoder_reader.read(&mut buf).unwrap();
    
    assert_eq!(result, 3);
    assert_eq!(&buf, b"ABC");
}

#[test]
fn test_read_with_non_empty_buffer() {
    struct MockEngine;

    impl Engine for MockEngine {
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
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            let b64_len = input.len();
            output[..b64_len / 4 * 3].copy_from_slice(&input[0..(b64_len / 4 * 3)]);
            Ok(DecodeMetadata { decoded_len: b64_len / 4 * 3, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let input_data = "QUJDQUJD"; // Base64 for "ABCABC"
    let mut reader = std::io::Cursor::new(input_data.as_bytes());
    let mut decoder_reader = DecoderReader::new(reader, &engine);
    
    let mut buf = [0u8; 6]; // Buffer for decoded output
    let result = decoder_reader.read(&mut buf).unwrap();
    
    assert_eq!(result, 6);
    assert_eq!(&buf, b"ABCABC");
}

