// Answer 0

#[test]
fn test_write_final_leftovers_no_extra_input() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len }
        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Ok(DecodeMetadata::default()) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    let engine = TestEngine;
    let writer = DummyWriter;
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.extra_input_occupied_len = 0; // precondition
    encoder_writer.delegate = Some(writer); // precondition

    let result = encoder_writer.write_final_leftovers();
}

#[test]
fn test_write_final_leftovers_with_output_buf() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len }
        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Ok(DecodeMetadata::default()) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    let engine = TestEngine;
    let writer = DummyWriter;
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.extra_input_occupied_len = 0; // precondition
    encoder_writer.delegate = Some(writer); // precondition
    encoder_writer.output_occupied_len = 512; // simulate some output being present

    let result = encoder_writer.write_final_leftovers();
}

