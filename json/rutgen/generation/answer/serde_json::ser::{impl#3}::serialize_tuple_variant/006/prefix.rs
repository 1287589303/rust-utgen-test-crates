// Answer 0

#[test]
fn test_serialize_tuple_variant_valid() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(_buf.len()) }
        fn write_all(&mut self, _: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn begin_object(&mut self, _writer: &mut TestWriter) -> Result<()> { Ok(()) }
        fn begin_object_key(&mut self, _writer: &mut TestWriter, _: bool) -> Result<()> { Ok(()) }
        fn end_object_key(&mut self, _writer: &mut TestWriter) -> Result<()> { Ok(()) }
        fn begin_object_value(&mut self, _writer: &mut TestWriter) -> Result<()> { Ok(()) }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;

    let serializer = &mut Serializer {
        writer,
        formatter,
    };

    serializer.serialize_tuple_variant("name", 0, "variant", 2).unwrap();
}

#[test]
fn test_serialize_tuple_variant_zero_length() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(_buf.len()) }
        fn write_all(&mut self, _: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn begin_object(&mut self, _writer: &mut TestWriter) -> Result<()> { Ok(()) }
        fn begin_object_key(&mut self, _writer: &mut TestWriter, _: bool) -> Result<()> { Ok(()) }
        fn end_object_key(&mut self, _writer: &mut TestWriter) -> Result<()> { Ok(()) }
        fn begin_object_value(&mut self, _writer: &mut TestWriter) -> Result<()> { Ok(()) }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;

    let serializer = &mut Serializer {
        writer,
        formatter,
    };

    serializer.serialize_tuple_variant("name", 1, "variant", 0).unwrap();
}

#[test]
#[should_panic]
fn test_serialize_tuple_variant_invalid() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(_buf.len()) }
        fn write_all(&mut self, _: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn begin_object(&mut self, _writer: &mut TestWriter) -> Result<()> { Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "error"))) }
        fn begin_object_key(&mut self, _writer: &mut TestWriter, _: bool) -> Result<()> { Ok(()) }
        fn end_object_key(&mut self, _writer: &mut TestWriter) -> Result<()> { Ok(()) }
        fn begin_object_value(&mut self, _writer: &mut TestWriter) -> Result<()> { Ok(()) }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;

    let serializer = &mut Serializer {
        writer,
        formatter,
    };

    serializer.serialize_tuple_variant("name", 1, "variant", 2).unwrap();
}

