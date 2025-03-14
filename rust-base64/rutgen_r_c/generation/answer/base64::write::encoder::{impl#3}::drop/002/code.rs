// Answer 0

#[test]
fn test_encoder_writer_drop_no_panicked() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len().min(output.len());
            output[..len].copy_from_slice(&input[..len]);
            len
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len // simple estimate
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

    use std::io::Cursor;

    let engine = TestEngine;
    let data = b"Hello, World!";
    let writer = Cursor::new(Vec::new());
    let mut encoder_writer = EncoderWriter::new(writer, &engine);

    encoder_writer.extra_input_occupied_len = 0; // Ensure no extra input
    encoder_writer.output_occupied_len = 0; // Ensure no occupied output
    encoder_writer.panicked = false; // Set panicked to false

    // Drop encoder_writer to trigger `drop`
    drop(encoder_writer);
}

#[test]
#[should_panic]
fn test_encoder_writer_drop_with_panicked() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            input.len().min(output.len()) // simple implementation
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len // simple estimate
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

    use std::io::Cursor;

    let engine = TestEngine;
    let writer = Cursor::new(Vec::new());
    let mut encoder_writer = EncoderWriter::new(writer, &engine);

    encoder_writer.panicked = true; // Set panicked to true to trigger panic on drop

    // Drop encoder_writer, which should panic
    drop(encoder_writer);
}

