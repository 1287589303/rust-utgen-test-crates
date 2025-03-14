// Answer 0

#[test]
fn test_write_final_leftovers_no_delegate() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len }
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> { Err(DecodeSliceError::InvalidInput) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    let engine = TestEngine;
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: None,
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let result = encoder_writer.write_final_leftovers();
    assert!(result.is_ok());
}

#[test]
fn test_write_final_leftovers_write_all_encoded_output_err() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len }
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> { Err(DecodeSliceError::InvalidInput) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> { Err(ErrorKind::Other.into()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    let engine = TestEngine;
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(DummyWriter),
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let result = encoder_writer.write_final_leftovers();
    assert!(result.is_err());
}

