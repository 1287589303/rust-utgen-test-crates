// Answer 0

#[test]
fn test_flush_decoded_buf_full_case() {
    struct MockEngine;
    struct MockReader;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { len: decode_estimate })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let mut decoder_reader = DecoderReader::new(MockReader, &engine);
    decoder_reader.decoded_len = 2;
    decoder_reader.decoded_chunk_buffer[0] = 1;
    decoder_reader.decoded_chunk_buffer[1] = 2;

    let mut buffer = [0u8; 3];
    let result = decoder_reader.flush_decoded_buf(&mut buffer);
}

#[test]
fn test_flush_decoded_buf_partial_case() {
    struct MockEngine;
    struct MockReader;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { len: decode_estimate })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let mut decoder_reader = DecoderReader::new(MockReader, &engine);
    decoder_reader.decoded_len = 1;
    decoder_reader.decoded_chunk_buffer[0] = 1;

    let mut buffer = [0u8; 2];
    let result = decoder_reader.flush_decoded_buf(&mut buffer);
}

