// Answer 0

#[test]
fn test_write_final_leftovers_no_delegate() -> Result<()> {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = ();
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { () }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Ok(DecodeMetadata::default()) }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let writer: Option<Vec<u8>> = None;
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: writer,
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    assert!(encoder_writer.write_final_leftovers().is_ok());
    Ok(())
}

