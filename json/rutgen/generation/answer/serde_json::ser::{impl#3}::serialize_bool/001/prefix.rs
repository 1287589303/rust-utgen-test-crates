// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter { output: Vec::new() };
    let mut serializer = Serializer { writer, formatter: CompactFormatter {} };
    let _ = serializer.serialize_bool(true);
}

#[test]
fn test_serialize_bool_false() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter { output: Vec::new() };
    let mut serializer = Serializer { writer, formatter: CompactFormatter {} };
    let _ = serializer.serialize_bool(false);
}

