// Answer 0

#[test]
fn test_encode_full_chunk_success() {
    struct MockSink;

    impl MockSink {
        fn new() -> Self {
            MockSink
        }
    }

    impl Sink for MockSink {
        type Error = ();

        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    struct MockEngine;

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = ();

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {}

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            todo!()
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    struct MockConfig;

    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            false
        }
    }

    let engine = MockEngine;
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink::new();
    let bytes = vec![1u8; 1024]; // Length is divisible by CHUNK_SIZE with no partial chunk

    let result = encoder.encode(&bytes, &mut sink);
    assert!(result.is_ok());
}

#[test]
fn test_encode_empty_input_success() {
    struct MockSink;

    impl MockSink {
        fn new() -> Self {
            MockSink
        }
    }

    impl Sink for MockSink {
        type Error = ();

        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    struct MockEngine;

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = ();

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {}

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            todo!()
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    struct MockConfig;

    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            false
        }
    }

    let engine = MockEngine;
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink::new();
    let bytes: Vec<u8> = vec![]; // Empty input

    let result = encoder.encode(&bytes, &mut sink);
    assert!(result.is_ok());
}

