// Answer 0

#[test]
fn test_drop_without_panicked() {
    struct MockEngine;
    struct MockWriter {
        data: Vec<u8>,
        panicked: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if self.panicked {
                return Err(io::Error::new(ErrorKind::Other, "panicked"));
            }
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let writer = MockWriter {
        data: Vec::new(),
        panicked: false,
    };

    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.extra_input_occupied_len = MIN_ENCODE_CHUNK_SIZE;
    encoder_writer.extra_input[..MIN_ENCODE_CHUNK_SIZE].copy_from_slice(b"abc");

    // This should not panic and should handle the leftovers correctly.
    drop(encoder_writer);

    assert_eq!(encoder_writer.output_occupied_len, 0);
}

#[test]
#[should_panic(expected = "panicked")]
fn test_drop_with_panicked() {
    struct MockEngine;
    struct MockWriter {
        data: Vec<u8>,
        panicked: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if self.panicked {
                panic!("panicked");
            }
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let mut writer = MockWriter {
        data: Vec::new(),
        panicked: true,
    };

    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.extra_input_occupied_len = MIN_ENCODE_CHUNK_SIZE;
    encoder_writer.extra_input[..MIN_ENCODE_CHUNK_SIZE].copy_from_slice(b"abc");

    // This will cause the panic in write_final_leftovers
    drop(encoder_writer);
}

