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

    let engine = MockEngine;
    let writer = MockWriter { data: Vec::new() };
    
    let encoder_writer = EncoderWriter::new(writer, &engine);
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

    let engine = MockEngine;
    let mut writer = MockWriter { data: Vec::new() };
    
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.finish().unwrap();
    
    let _inner_writer = encoder_writer.into_inner(); // Call to finish before this line
    // The panic should occur on the next call to into_inner
    let _inner_writer_again = encoder_writer.into_inner();
}

