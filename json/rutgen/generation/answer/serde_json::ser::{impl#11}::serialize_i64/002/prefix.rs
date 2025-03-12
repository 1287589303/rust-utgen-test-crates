// Answer 0

#[test]
fn test_serialize_i64_boundary_min() {
    struct MockWriter;
    struct MockFormatter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error)
        }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> { Ok(()) }
        fn write_i64(&mut self, _writer: &mut dyn io::Write, _value: i64) -> Result<()> { Err(Error) }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> { Ok(()) }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_i64(i64::MIN);
}

#[test]
fn test_serialize_i64_boundary_max() {
    struct MockWriter;
    struct MockFormatter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error)
        }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> { Ok(()) }
        fn write_i64(&mut self, _writer: &mut dyn io::Write, _value: i64) -> Result<()> { Err(Error) }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> { Ok(()) }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_i64(i64::MAX);
}

#[test]
fn test_serialize_i64_invalid_value() {
    struct MockWriter;
    struct MockFormatter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error)
        }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> { Ok(()) }
        fn write_i64(&mut self, _writer: &mut dyn io::Write, _value: i64) -> Result<()> { Err(Error) }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> { Ok(()) }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_i64(-1);
}

