// Answer 0

fn test_write_all_encoded_output_interrupt_error() -> Result<()> {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output.copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
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

    use std::io::{self, Write};

    struct MockWriter {
        should_fail: bool,
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_fail {
                Err(io::Error::new(ErrorKind::Other, "mock error"))
            } else {
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let mut writer = MockWriter { should_fail: true };
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 1, // ensure > 0
        panicked: false,
    };

    let result = encoder_writer.write_all_encoded_output();
    assert!(result.is_err());

    Ok(())
}

fn test_write_all_encoded_output_interrupted_error() -> Result<()> {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output.copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
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

    use std::io::{self, Write};

    struct MockWriter {
        errors: Vec<ErrorKind>, // List of errors to simulate
    }

    impl Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            if let Some(error) = self.errors.pop() {
                match error {
                    ErrorKind::Interrupted => Err(io::Error::new(ErrorKind::Interrupted, "interrupted")),
                    _ => Err(io::Error::new(error, "mock error")),
                }
            } else {
                Ok(1)
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let mut writer = MockWriter {
        errors: vec![ErrorKind::Interrupted, ErrorKind::Other],
    };
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 1, // ensure > 0
        panicked: false,
    };

    let result = encoder_writer.write_all_encoded_output();
    assert!(result.is_err());

    Ok(())
}

