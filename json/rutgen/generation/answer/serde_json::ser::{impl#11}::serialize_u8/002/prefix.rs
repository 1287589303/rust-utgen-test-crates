// Answer 0

#[test]
fn test_serialize_u8_success() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn begin_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }
        fn write_u8(&mut self, _writer: &mut TestWriter, _value: u8) -> Result<()> {
            Err(Error)
        }
        fn end_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };

    let key_serializer = MapKeySerializer { ser: &mut serializer };
    let result = key_serializer.serialize_u8(100);
}

#[test]
fn test_serialize_u8_write_error() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn begin_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }
        fn write_u8(&mut self, _writer: &mut TestWriter, _value: u8) -> Result<()> {
            Err(Error)
        }
        fn end_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };

    let key_serializer = MapKeySerializer { ser: &mut serializer };
    
    for value in [0u8, 255u8].iter() {
        let result = key_serializer.serialize_u8(*value);
    }
}

