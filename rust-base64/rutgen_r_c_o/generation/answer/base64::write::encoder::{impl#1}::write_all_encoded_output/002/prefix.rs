// Answer 0

#[test]
fn test_write_all_encoded_output_interrupted_error() {
    struct MockEngine;
    struct MockWriter {
        should_interrupt: bool,
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_interrupt {
                return Err(io::Error::from(ErrorKind::Interrupted));
            }
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    let engine = MockEngine;
    let mut writer = MockWriter { should_interrupt: true, buffer: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.output_occupied_len = 10; // arbitrary value > 0

    let _ = encoder_writer.write_all_encoded_output();
}

#[test]
fn test_write_all_encoded_output_other_error() {
    struct MockEngine;
    struct MockWriter {
        should_fail: bool,
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_fail {
                return Err(io::Error::from(ErrorKind::Other));
            }
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    let engine = MockEngine;
    let mut writer = MockWriter { should_fail: true, buffer: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.output_occupied_len = 10; // arbitrary value > 0

    let _ = encoder_writer.write_all_encoded_output();
}

#[test]
fn test_write_all_encoded_output_successful_write() {
    struct MockEngine;
    struct MockWriter {
        written: usize,
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.written += buf.len();
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    let engine = MockEngine;
    let mut writer = MockWriter { written: 0, buffer: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.output_occupied_len = 10; // arbitrary value > 0

    let _ = encoder_writer.write_all_encoded_output();
    assert_eq!(encoder_writer.output_occupied_len, 0);
}

