// Answer 0

#[test]
fn test_new_chunked_encoder_with_valid_engine() {
    struct MockConfig;
    struct MockDecodeEstimate;
    
    struct MockEngine;

    impl Config for MockConfig {}
    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = MockDecodeEstimate;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate {
            MockDecodeEstimate
        }

        fn internal_decode(
            &self, 
            _: &[u8], 
            _: &mut [u8], 
            _: Self::DecodeEstimate
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata)
        }
        
        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    let engine = MockEngine;
    let encoder = ChunkedEncoder::new(&engine);
}

#[test]
fn test_new_chunked_encoder_with_another_valid_engine() {
    struct AnotherMockConfig;
    struct AnotherMockDecodeEstimate;

    struct AnotherMockEngine;

    impl Config for AnotherMockConfig {}
    impl Engine for AnotherMockEngine {
        type Config = AnotherMockConfig;
        type DecodeEstimate = AnotherMockDecodeEstimate;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate {
            AnotherMockDecodeEstimate
        }

        fn internal_decode(
            &self, 
            _: &[u8], 
            _: &mut [u8], 
            _: Self::DecodeEstimate
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata)
        }

        fn config(&self) -> &Self::Config {
            &AnotherMockConfig
        }
    }

    let another_engine = AnotherMockEngine;
    let encoder = ChunkedEncoder::new(&another_engine);
}

