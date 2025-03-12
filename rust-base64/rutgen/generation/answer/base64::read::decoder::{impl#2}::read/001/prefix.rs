// Answer 0

#[test]
fn test_read_empty_buf() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: decode_estimate })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let reader: &[u8] = &[];
    let mut decoder_reader = DecoderReader::new(reader, &engine);
    let mut buf: [u8; 0] = [];

    let result = decoder_reader.read(&mut buf);
}

