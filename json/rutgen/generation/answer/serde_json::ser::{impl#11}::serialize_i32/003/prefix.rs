// Answer 0

#[test]
fn test_serialize_i32_positive() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, writer: &mut MockWriter) -> Result<()> {
            writer.write(&[b'"'])?;
            Ok(())
        }

        fn write_i32(&mut self, writer: &mut MockWriter, value: i32) -> Result<()> {
            let bytes = value.to_string().as_bytes();
            writer.write(bytes)?;
            Ok(())
        }

        fn end_string(&mut self, writer: &mut MockWriter) -> Result<()> {
            writer.write(&[b'"'])?;
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;

    let serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    let _ = serializer.serialize_i32(42);
}

#[test]
fn test_serialize_i32_negative() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, writer: &mut MockWriter) -> Result<()> {
            writer.write(&[b'"'])?;
            Ok(())
        }

        fn write_i32(&mut self, writer: &mut MockWriter, value: i32) -> Result<()> {
            let bytes = value.to_string().as_bytes();
            writer.write(bytes)?;
            Ok(())
        }

        fn end_string(&mut self, writer: &mut MockWriter) -> Result<()> {
            writer.write(&[b'"'])?;
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;

    let serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    let _ = serializer.serialize_i32(-42);
}

#[test]
fn test_serialize_i32_boundary() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, writer: &mut MockWriter) -> Result<()> {
            writer.write(&[b'"'])?;
            Ok(())
        }

        fn write_i32(&mut self, writer: &mut MockWriter, value: i32) -> Result<()> {
            let bytes = value.to_string().as_bytes();
            writer.write(bytes)?;
            Ok(())
        }

        fn end_string(&mut self, writer: &mut MockWriter) -> Result<()> {
            writer.write(&[b'"'])?;
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;

    let serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    let _ = serializer.serialize_i32(i32::MIN);
    let _ = serializer.serialize_i32(i32::MAX);
}

