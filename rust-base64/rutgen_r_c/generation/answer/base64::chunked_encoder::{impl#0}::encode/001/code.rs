// Answer 0

#[test]
fn test_encode_partial_chunk_with_padding_error() {
    struct TestConfig {
        padding: bool,
    }

    impl Config for TestConfig {
        fn encode_padding(&self) -> bool {
            self.padding
        }
    }

    struct TestEngine {
        config: TestConfig,
    }

    impl Engine for TestEngine {
        type Config = TestConfig;
        type DecodeEstimate = usize; // Placeholder

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Simple encoding for test
            let len = input.len();
            output[..len].copy_from_slice(input);
            len
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len // Placeholder
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
            &self.config
        }
    }

    struct TestSink {
        error: Option<()>,
    }

    impl TestSink {
        fn new(error: Option<()>) -> Self {
            TestSink { error }
        }
    }

    impl Sink for TestSink {
        type Error = ();
        
        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error> {
            self.error.clone().ok_or(())
        }
    }

    let config = TestConfig { padding: true };
    let engine = TestEngine { config };
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = TestSink::new(Some(()));
    let data: Vec<u8> = vec![1, 2, 3]; // Shorter than CHUNK_SIZE

    let result = encoder.encode(&data, &mut sink);
    assert!(result.is_err());
}

#[test]
fn test_encode_full_chunk_without_padding_error() {
    struct TestConfig {
        padding: bool,
    }

    impl Config for TestConfig {
        fn encode_padding(&self) -> bool {
            self.padding
        }
    }

    struct TestEngine {
        config: TestConfig,
    }

    impl Engine for TestEngine {
        type Config = TestConfig;
        type DecodeEstimate = usize; // Placeholder

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Simple encoding for test
            let len = input.len();
            output[..len].copy_from_slice(input);
            len
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len // Placeholder
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
            &self.config
        }
    }

    struct TestSink {
        error: Option<()>,
    }

    impl TestSink {
        fn new(error: Option<()>) -> Self {
            TestSink { error }
        }
    }

    impl Sink for TestSink {
        type Error = ();
        
        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error> {
            self.error.clone().ok_or(())
        }
    }

    let config = TestConfig { padding: false };
    let engine = TestEngine { config };
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = TestSink::new(Some(()));
    let data: Vec<u8> = vec![0; 1024]; // Length is multiple of CHUNK_SIZE

    let result = encoder.encode(&data, &mut sink);
    assert!(result.is_err());
}

