// Answer 0

#[test]
fn test_encode_partial_chunk_one_byte() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Mock implementation
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &MockConfig {}
        }
    }

    struct MockConfig;
    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    struct MockSink {
        data: Vec<u8>,
    }

    impl MockSink {
        fn new() -> Self {
            MockSink { data: Vec::new() }
        }
        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), ()> {
            self.data.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = MockEngine {};
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink::new();
    let bytes = vec![1; (1024 / 4 * 3) + 1]; // CHUNK_SIZE + 1
    let result = encoder.encode(&bytes, &mut sink);
    assert!(result.is_ok());
}

#[test]
fn test_encode_partial_chunk_two_bytes() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &MockConfig {}
        }
    }

    struct MockConfig;
    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    struct MockSink {
        data: Vec<u8>,
    }

    impl MockSink {
        fn new() -> Self {
            MockSink { data: Vec::new() }
        }
        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), ()> {
            self.data.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = MockEngine {};
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink::new();
    let bytes = vec![1; (1024 / 4 * 3) + 2]; // CHUNK_SIZE + 2
    let result = encoder.encode(&bytes, &mut sink);
    assert!(result.is_ok());
}

#[test]
fn test_encode_partial_chunk_three_bytes() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &MockConfig {}
        }
    }

    struct MockConfig;
    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    struct MockSink {
        data: Vec<u8>,
    }

    impl MockSink {
        fn new() -> Self {
            MockSink { data: Vec::new() }
        }
        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), ()> {
            self.data.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = MockEngine {};
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink::new();
    let bytes = vec![1; (1024 / 4 * 3) + 3]; // CHUNK_SIZE + 3
    let result = encoder.encode(&bytes, &mut sink);
    assert!(result.is_ok());
}

#[test]
fn test_encode_partial_chunk_max() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &MockConfig {}
        }
    }

    struct MockConfig;
    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    struct MockSink {
        data: Vec<u8>,
    }

    impl MockSink {
        fn new() -> Self {
            MockSink { data: Vec::new() }
        }
        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), ()> {
            self.data.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = MockEngine {};
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink::new();
    let bytes = vec![1; 1023]; // BUF_SIZE - 1
    let result = encoder.encode(&bytes, &mut sink);
    assert!(result.is_ok());
}

