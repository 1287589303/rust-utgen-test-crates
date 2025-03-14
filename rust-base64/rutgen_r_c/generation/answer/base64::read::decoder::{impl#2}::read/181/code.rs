// Answer 0

fn test_decoder_reader_read_eof_with_buffer() -> Result<(), Box<dyn std::error::Error>> {
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
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::OutputSliceTooSmall)
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let input_data = b"SGVsbG8gV29ybGQ="; // Base64 of "Hello World"
    
    let cursor = std::io::Cursor::new(input_data);
    let mut decoder = DecoderReader::new(cursor, &engine);

    let mut buffer = vec![0; 1024];
    let bytes_read = decoder.read(&mut buffer)?;

    assert_eq!(bytes_read, 0);
    Ok(())
}

fn test_decoder_reader_read_partial_data() -> Result<(), Box<dyn std::error::Error>> {
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
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 3, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let input_data = b"SGVsbG8g"; // Base64 of "Hello g"
    
    let cursor = std::io::Cursor::new(input_data);
    let mut decoder = DecoderReader::new(cursor, &engine);
    
    let mut buffer = vec![0; 3];
    let bytes_read = decoder.read(&mut buffer)?;

    assert_eq!(bytes_read, 3);
    assert_eq!(&buffer[..bytes_read], b"Hel");
    Ok(())
} 

fn test_decoder_reader_read_invalid_data() -> Result<(), Box<dyn std::error::Error>> {
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
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::OutputSliceTooSmall)
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let input_data = b"!@#$%^"; // Invalid Base64 input
    
    let cursor = std::io::Cursor::new(input_data);
    let mut decoder = DecoderReader::new(cursor, &engine);
    
    let mut buffer = vec![0; 3];
    let result = decoder.read(&mut buffer);
    
    assert!(result.is_err());
    Ok(())
} 

// Test runner
fn main() {
    let _ = test_decoder_reader_read_eof_with_buffer().expect("Test failed");
    let _ = test_decoder_reader_read_partial_data().expect("Test failed");
    let _ = test_decoder_reader_read_invalid_data().expect("Test failed");
}

