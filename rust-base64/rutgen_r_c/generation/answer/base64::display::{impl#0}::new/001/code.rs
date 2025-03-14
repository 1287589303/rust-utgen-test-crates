// Answer 0

#[test]
fn test_base64_display_new() {
    struct TestConfig;
    struct TestDecodeEstimate;

    impl Config for TestConfig {}
    impl DecodeEstimate for TestDecodeEstimate {}

    struct TestEngine;

    impl Engine for TestEngine {
        type Config = TestConfig;
        type DecodeEstimate = TestDecodeEstimate;
        
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate {
            TestDecodeEstimate {}
        }
        
        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::new())
        }
        
        fn config(&self) -> &Self::Config {
            &TestConfig {}
        }
    }

    let bytes: &[u8] = b"test";
    let engine = TestEngine;

    let display = Base64Display::new(bytes, &engine);

    assert_eq!(display.bytes, bytes);
    assert!(std::mem::size_of_val(&display.chunked_encoder) > 0);
}

#[test]
fn test_base64_display_new_empty_bytes() {
    struct TestConfig;
    struct TestDecodeEstimate;

    impl Config for TestConfig {}
    impl DecodeEstimate for TestDecodeEstimate {}

    struct TestEngine;

    impl Engine for TestEngine {
        type Config = TestConfig;
        type DecodeEstimate = TestDecodeEstimate;
        
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate {
            TestDecodeEstimate {}
        }
        
        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::new())
        }
        
        fn config(&self) -> &Self::Config {
            &TestConfig {}
        }
    }

    let bytes: &[u8] = &[];
    let engine = TestEngine;

    let display = Base64Display::new(bytes, &engine);

    assert_eq!(display.bytes, bytes);
    assert!(std::mem::size_of_val(&display.chunked_encoder) > 0);
}

