// Answer 0

#[test]
fn test_serialize_bool_write_fail_true() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<()> {
            Err(Error)
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = key_serializer.serialize_bool(true);
}

#[test]
fn test_serialize_bool_write_fail_false() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<()> {
            Err(Error)
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = key_serializer.serialize_bool(false);
}

