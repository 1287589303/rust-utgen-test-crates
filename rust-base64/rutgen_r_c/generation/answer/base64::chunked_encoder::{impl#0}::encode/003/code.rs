// Answer 0


#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockConfig {
        padding: bool,
    }

    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            self.padding
        }
    }

    struct MockEngine {
        config: MockConfig,
    }

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = usize; // Placeholder type

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Mock encoding - just copy input to output
            let len = input.len().min(output.len());
            output[..len].copy_from_slice(&input[..len]);
            len
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0 // Dummy implementation
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
    
    struct MockSink {
        error: Option<SinkError>,
    }

    impl Sink for MockSink {
        type Error = SinkError;

        fn write_encoded_bytes(&mut self, _encoded_bytes: &[u8]) -> Result<(), Self::Error> {
            if let Some(err) = self.error.take() {
                Err(err)
            } else {
                Ok(())
            }
        }
    }

    #[derive(Debug)]
    struct SinkError;

    #[test]
    fn test_encode_partial_chunk_no_padding_error() {
        let engine = MockEngine {
            config: MockConfig { padding: false },
        };
        let encoder = ChunkedEncoder::new(&engine);
        let mut sink = MockSink { error: None };
        let input_data = [1, 2, 3, 4, 5, 6]; // Example data, will be chunked
    
        // Should succeed without padding
        let result = encoder.encode(&input_data, &mut sink);
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_partial_chunk_no_padding_sink_error() {
        let engine = MockEngine {
            config: MockConfig { padding: false },
        };
        let encoder = ChunkedEncoder::new(&engine);
        let mut sink = MockSink { error: Some(SinkError) };
        let input_data = [1, 2, 3, 4, 5, 6]; // Example data, will be chunked

        // Should return error due to sink error
        let result = encoder.encode(&input_data, &mut sink);
        assert!(result.is_err());
    }
}


