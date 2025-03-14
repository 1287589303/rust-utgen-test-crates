// Answer 0

#[test]
fn test_encode_with_full_chunks_no_padding() {
    struct MockEngine;
    
    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = ();
        
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len(); // Mock the encoding to just return the length
            output[..len].copy_from_slice(input);
            len
        }
        
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            ()
        }
        
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<(), DecodeSliceError> {
            Ok(())
        }
        
        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }
    
    struct MockConfig;
    
    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            false // No padding needed for this test case
        }
    }
    
    struct MockSink {
        write_called: bool,
    }
    
    impl MockSink {
        fn new() -> Self {
            Self { write_called: false }
        }
    }
    
    impl Sink for MockSink {
        type Error = ();
        
        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error> {
            self.write_called = true; // Simulate a successful write
            Ok(())
        }
    }
    
    let engine = MockEngine;
    let chunked_encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink::new();
    let data = vec![0; 1024]; // 1024 bytes of data which makes it a full chunk

    let result = chunked_encoder.encode(&data, &mut sink);
    
    assert!(result.is_ok());
    assert!(sink.write_called);
}

#[test]
fn test_encode_with_partial_chunk_and_padding() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = ();
        
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len(); // Mock the encoding to return the length
            output[..len].copy_from_slice(input);
            len
        }
        
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            ()
        }
        
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<(), DecodeSliceError> {
            Ok(())
        }
        
        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    struct MockConfig;

    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            true // Padding is needed for this test case
        }
    }

    struct MockSink {
        error_on_write: bool,
    }

    impl MockSink {
        fn new(error_on_write: bool) -> Self {
            Self { error_on_write }
        }
    }

    impl Sink for MockSink {
        type Error = ();

        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error> {
            if self.error_on_write {
                Err(()) // Simulate an error on write
            } else {
                Ok(())
            }
        }
    }

    let engine = MockEngine;
    let chunked_encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink::new(true); // Simulate error on write
    let data = vec![0; 123]; // 123 bytes of data for the partial chunk

    let result = chunked_encoder.encode(&data, &mut sink);
    
    assert!(result.is_err());
}

