// Answer 0

#[test]
fn test_read_with_non_empty_buffer_at_eof() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0, padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let input_data: &[u8] = b"SGVsbG8sIFdvcmxkIQ=="; // Base64 for "Hello, World!"
    let cursor = std::io::Cursor::new(input_data);
    let mut decoder = DecoderReader::new(cursor, &engine);

    let mut buf = vec![0; 6]; // buf.length > 3
    decoder.b64_offset = BUF_SIZE; // self.b64_offset == BUF_SIZE
    decoder.b64_len = BUF_SIZE; // self.b64_len == BUF_SIZE
    decoder.decoded_len = 0; // self.decoded_len == 0
    decoder.decoded_offset = DECODED_CHUNK_SIZE; // self.decoded_offset == DECODED_CHUNK_SIZE
    decoder.decoded_chunk_buffer = [72, 101, 108]; // Decoded 'Hel'

    let _ = decoder.read(&mut buf);
}

#[test]
fn test_read_with_buffer_size_exactly_chunk_size() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 3, padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let input_data: &[u8] = b"SGVsbG8sIFdvcmxkIQ=="; // Base64 for "Hello, World!"
    let cursor = std::io::Cursor::new(input_data);
    let mut decoder = DecoderReader::new(cursor, &engine);

    let mut buf = vec![0; 3]; // buf.length > 3
    decoder.b64_offset = BUF_SIZE; // self.b64_offset == BUF_SIZE
    decoder.b64_len = BUF_SIZE; // self.b64_len == BUF_SIZE
    decoder.decoded_len = 0; // self.decoded_len == 0
    decoder.decoded_offset = DECODED_CHUNK_SIZE; // self.decoded_offset == DECODED_CHUNK_SIZE
    decoder.decoded_chunk_buffer = [72, 101, 108]; // Decoded 'Hel'

    let _ = decoder.read(&mut buf);
}

#[test]
fn test_read_with_unconsumed_bytes() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 3, padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let input_data: &[u8] = b"SGVsbG8sIFdvcmxkIQ=="; // Base64 for "Hello, World!"
    let cursor = std::io::Cursor::new(input_data);
    let mut decoder = DecoderReader::new(cursor, &engine);

    let mut buf = vec![0; 9]; // buf.length > 3
    decoder.b64_offset = BUF_SIZE; // self.b64_offset == BUF_SIZE
    decoder.b64_len = BUF_SIZE; // self.b64_len == BUF_SIZE
    decoder.decoded_len = 0; // self.decoded_len == 0
    decoder.decoded_offset = DECODED_CHUNK_SIZE; // self.decoded_offset == DECODED_CHUNK_SIZE
    decoder.decoded_chunk_buffer = [72, 101, 108]; // Decoded 'Hel'

    let _ = decoder.read(&mut buf);
}

