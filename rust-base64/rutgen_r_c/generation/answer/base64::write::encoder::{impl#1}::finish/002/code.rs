// Answer 0

#[test]
fn test_finish_with_valid_delegate() {
    struct MockEngine;
    struct MockWriter {
        finished: bool,
        data_written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data_written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = cmp::min(input.len(), output.len());
            output[..len].copy_from_slice(&input[..len]);
            len
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let mut writer = MockWriter {
        finished: false,
        data_written: Vec::new(),
    };

    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.extra_input_occupied_len = 0; // Simulate no extra input for this case
    encoder_writer.output_occupied_len = 0; // Reset output before finishing

    let result = encoder_writer.finish();

    assert!(result.is_ok());
    // Ensure the original writer is returned
    let writer = result.unwrap();
    assert!(writer.data_written.len() > 0); // Check data was written
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_finish_panic_on_double_finish() {
    struct MockEngine;
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> { Ok(0) }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;

    let mut encoder_writer = EncoderWriter::new(MockWriter, &engine);
    encoder_writer.finish().unwrap(); // Call finish once

    encoder_writer.finish().unwrap(); // This should panic
}

