// Answer 0

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_finish_when_delegate_is_none() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Ok(DecodeMetadata {}) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    let engine = MockEngine {};
    let writer = MockWriter {};
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
  
    encoder_writer.delegate = None;

    let _ = encoder_writer.finish();
}

#[test]
#[should_panic(expected = "Writer must be present")]
fn test_finish_when_writer_is_none() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Ok(DecodeMetadata {}) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    let engine = MockEngine {};
    let mut encoder_writer = EncoderWriter::new(MockWriter {}, &engine);
  
    // Directly nullifying delegate to simulate condition
    encoder_writer.delegate = Some(MockWriter {}) ;
    encoder_writer.finish().unwrap(); // Call finish to set delegate to None
  
    // Now simulate finish when delegate is already taken
    let _ = encoder_writer.finish();
}

