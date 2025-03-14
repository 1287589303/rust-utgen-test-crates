// Answer 0

#[test]
fn test_decoder_reader_debug_fmt() {
    struct MockConfig;
    struct MockDecodeEstimate;

    impl Config for MockConfig {}
    impl DecodeEstimate for MockDecodeEstimate {}

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
            Ok(DecodeMetadata::default())
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    let engine = MockEngine;
    let mut b64_buffer = [0u8; BUF_SIZE];
    let mut decoded_chunk_buffer = [0u8; DECODED_CHUNK_SIZE];
    let decoder_reader = DecoderReader {
        engine: &engine,
        inner: std::io::empty(),
        b64_buffer,
        b64_offset: 0,
        b64_len: 0,
        decoded_chunk_buffer,
        decoded_offset: 0,
        decoded_len: 0,
        input_consumed_len: 0,
        padding_offset: None,
    };

    let mut result = String::new();
    let _ = write!(&mut result, "{:?}", decoder_reader);
    assert!(result.contains("DecoderReader"));
}

