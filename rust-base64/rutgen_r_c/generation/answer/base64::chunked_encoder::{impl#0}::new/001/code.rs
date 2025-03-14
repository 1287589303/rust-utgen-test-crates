// Answer 0

#[test]
fn test_chunked_encoder_new() {
    struct DummyEngine;
    
    impl Config for DummyEngine {}
    
    struct DummyDecodeEstimate;
    
    impl DecodeEstimate for DummyDecodeEstimate {}

    impl Engine for DummyEngine {
        type Config = DummyEngine;
        type DecodeEstimate = DummyDecodeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            DummyDecodeEstimate
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }

        fn config(&self) -> &Self::Config {
            self
        }
    }

    let engine = DummyEngine;
    let encoder = ChunkedEncoder::new(&engine);
    assert_eq!(encoder.engine as *const _, &engine as *const _);
}

