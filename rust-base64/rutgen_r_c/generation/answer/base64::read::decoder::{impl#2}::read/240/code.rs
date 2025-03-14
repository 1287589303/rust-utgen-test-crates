// Answer 0

#[test]
fn test_read_with_empty_buf() {
    struct DummyEngine;
    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len / 4 * 3 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Ok(DecodeMetadata { decoded_len: 0, padding_offset: None }) }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = DummyEngine;
    let mut reader = DecoderReader::new(std::io::empty(), &engine);
    let mut buf = [0u8; 0]; // empty buffer

    let result = reader.read(&mut buf);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_read_when_buf_size_exceeded() {
    struct DummyEngine;
    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len / 4 * 3 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Ok(DecodeMetadata { decoded_len: 0, padding_offset: None }) }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = DummyEngine;
    let mut reader = DecoderReader::new(std::io::empty(), &engine);
    reader.b64_offset = BUF_SIZE; // b64_offset == BUF_SIZE
    reader.b64_len = BUF_SIZE; // setting b64_len such that it exceeds the buffer size

    let mut buf = [0u8; 4]; // valid buffer size
    let result = reader.read(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_read_with_partially_full_buffer() {
    struct DummyEngine;
    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len / 4 * 3 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Ok(DecodeMetadata { decoded_len: 3, padding_offset: None }) }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = DummyEngine;
    let mut reader = DecoderReader::new(std::io::empty(), &engine);
    reader.b64_offset = BUF_SIZE; // b64_offset == BUF_SIZE
    reader.b64_len = 0; // no buffered data
    reader.decoded_len = 3; // pretend we have decoded bytes ready

    let mut buf = [0u8; 4]; // valid buffer size
    let result = reader.read(&mut buf);
    assert_eq!(result.unwrap(), 3);
}

