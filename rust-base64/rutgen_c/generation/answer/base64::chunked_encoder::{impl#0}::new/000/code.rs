// Answer 0

#[test]
fn test_chunked_encoder_new() {
    struct MockConfig;
    struct MockDecodeEstimate;

    struct MockEngine;
    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = MockDecodeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            MockDecodeEstimate
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    let engine = MockEngine;
    let encoder = ChunkedEncoder::new(&engine);
    assert_eq!(std::ptr::addr_of!(encoder.engine), std::ptr::addr_of!(engine));
}

