// Answer 0

#[test]
fn test_encode_with_empty_bytes() {
    struct MockEngine {
        padding: bool,
    }

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], output: &mut [u8]) -> usize {
            // Empty input, nothing is written to output
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &MockConfig { encode_padding: self.padding }
        }
    }

    struct MockConfig {
        encode_padding: bool,
    }

    struct MockSink {
        written: Vec<u8>,
    }

    impl MockSink {
        fn new() -> Self {
            MockSink { written: Vec::new() }
        }

        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), ()> {
            self.written.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = MockEngine { padding: false };
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink::new();
    let result = encoder.encode(&[], &mut sink);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_encode_with_partial_chunk() {
    struct MockEngine {
        padding: bool,
    }

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Mock encoding: copy input to output for testing
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &MockConfig { encode_padding: self.padding }
        }
    }

    struct MockConfig {
        encode_padding: bool,
    }

    struct MockSink {
        written: Vec<u8>,
    }

    impl MockSink {
        fn new() -> Self {
            MockSink { written: Vec::new() }
        }

        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), ()> {
            self.written.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = MockEngine { padding: true };
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink::new();

    let test_bytes = vec![1, 2, 3, 4, 5]; // Length is 5, leading to a partial chunk
    let result = encoder.encode(&test_bytes, &mut sink);
    assert_eq!(result, Ok(()));
    assert_eq!(sink.written.len(), 5);
}

