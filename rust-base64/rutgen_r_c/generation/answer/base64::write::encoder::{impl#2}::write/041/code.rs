// Answer 0

#[test]
fn test_write_empty_input_with_delegate_none() {
    struct DummyEngine;
    impl Engine for DummyEngine {
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
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = DummyEngine;
    let writer = Vec::new();
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: None,
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let result = encoder_writer.write(&[]);
    assert_eq!(result, Ok(0));
}

#[test]
#[should_panic(expected = "Cannot write more after calling finish()")]
fn test_write_non_empty_input_with_delegate_none() {
    struct DummyEngine;
    impl Engine for DummyEngine {
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
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = DummyEngine;
    let writer = Vec::new();
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: None,
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let _ = encoder_writer.write(&[1, 2, 3]);
}

