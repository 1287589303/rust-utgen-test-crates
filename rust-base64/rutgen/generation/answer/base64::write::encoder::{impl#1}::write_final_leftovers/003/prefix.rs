// Answer 0

#[test]
fn test_write_final_leftovers_with_ok_first_write_and_err_second_write() {
    struct MockWriter {
        written: Vec<u8>,
        should_fail_on_second_write: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_fail_on_second_write && !self.written.is_empty() {
                return Err(io::Error::new(ErrorKind::Other, "write failed"));
            }
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Mock encoding: simply copy input to output and return length
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }

        #[cfg(any(feature = "alloc", test))]
        fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
            input.as_ref().to_vec()
        }
    }

    let mut writer = MockWriter {
        written: Vec::new(),
        should_fail_on_second_write: true,
    };

    let engine = MockEngine;

    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [1, 2, 3], // mock extra input of length MIN_ENCODE_CHUNK_SIZE
        extra_input_occupied_len: 3,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let _ = encoder_writer.write_final_leftovers();
}

#[test]
fn test_write_final_leftovers_with_ok_first_write_and_exceed_max_input() {
    struct MockWriter {
        written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Mock encoding: simply copy input to output and return length
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }

        #[cfg(any(feature = "alloc", test))]
        fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
            input.as_ref().to_vec().iter().map(|&b| b + 1).collect()
        }
    }

    let mut writer = MockWriter {
        written: Vec::new(),
    };

    let engine = MockEngine;

    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [1, 2, 3],
        extra_input_occupied_len: 3,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let _ = encoder_writer.write_final_leftovers();
}

