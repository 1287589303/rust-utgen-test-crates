// Answer 0

#[test]
fn test_decoder_reader_debug_fmt() {
    struct TestEngine;
    struct TestConfig;
    struct TestDecodeEstimate;

    impl crate::Engine for TestEngine {
        type Config = TestConfig;
        type DecodeEstimate = TestDecodeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            TestDecodeEstimate
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<crate::DecodeMetadata, crate::DecodeSliceError> {
            Ok(crate::DecodeMetadata { ..Default::default() })
        }

        fn config(&self) -> &Self::Config {
            &TestConfig
        }
    }

    let engine = TestEngine;
    let reader: &[u8] = b"some base64 data";
    let decoder_reader = crate::DecoderReader {
        engine: &engine,
        inner: reader,
        b64_buffer: [0; crate::BUF_SIZE],
        b64_offset: 0,
        b64_len: 0,
        decoded_chunk_buffer: [0; crate::DECODED_CHUNK_SIZE],
        decoded_offset: 0,
        decoded_len: 0,
        input_consumed_len: 0,
        padding_offset: None,
    };

    let mut result = String::new();
    let _ = writeln!(result, "{:?}", decoder_reader);
    assert!(result.contains("DecoderReader"));
}

