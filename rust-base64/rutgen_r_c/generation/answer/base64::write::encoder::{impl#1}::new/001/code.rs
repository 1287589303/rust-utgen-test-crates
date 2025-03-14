// Answer 0

#[test]
fn test_encoder_writer_new_with_valid_delegate_and_engine() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<(), ()> {
            Ok(())
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let mock_engine = MockEngine;
    let mock_writer: Vec<u8> = Vec::new();

    let encoder_writer = EncoderWriter::new(mock_writer, &mock_engine);

    assert!(encoder_writer.delegate.is_some());
    assert_eq!(encoder_writer.extra_input, [0u8; MIN_ENCODE_CHUNK_SIZE]);
    assert_eq!(encoder_writer.extra_input_occupied_len, 0);
    assert_eq!(encoder_writer.output, [0u8; BUF_SIZE]);
    assert_eq!(encoder_writer.output_occupied_len, 0);
    assert_eq!(encoder_writer.panicked, false);
}

