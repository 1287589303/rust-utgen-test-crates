// Answer 0

#[test]
fn test_into_inner_before_finish() {
    struct MockEngine;
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mock_writer = MockWriter { data: Vec::new() };
    let engine = MockEngine;

    let mut encoder_writer = EncoderWriter::new(mock_writer, &engine);
    let inner_writer = encoder_writer.into_inner();

    assert!(inner_writer.data.is_empty());
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_into_inner_after_finish() {
    struct MockEngine;
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mock_writer = MockWriter { data: Vec::new() };
    let engine = MockEngine;

    let mut encoder_writer = EncoderWriter::new(mock_writer, &engine);
    encoder_writer.finish().unwrap(); // Call finish to complete writing
    let _ = encoder_writer.into_inner(); // Get inner writer to avoid panic
    encoder_writer.into_inner(); // This should panic
}

#[test]
fn test_into_inner_with_data() {
    struct MockEngine;
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mock_writer = MockWriter { data: Vec::new() };
    let engine = MockEngine;

    let mut encoder_writer = EncoderWriter::new(mock_writer, &engine);
    encoder_writer.write_all_encoded_output().unwrap(); // Assume some data is encoded

    let inner_writer = encoder_writer.into_inner();
    assert!(!inner_writer.data.is_empty());
}

