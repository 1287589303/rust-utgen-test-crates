// Answer 0

#[test]
fn test_serialize_i128_min() {
    struct TestWriter;
    
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> { Ok(()) }
        fn write_i128(&mut self, _writer: &mut impl io::Write, _value: i128) -> Result<()> { Ok(()) }
        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> { Ok(()) }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    serializer.serialize_i128(i128::MIN).unwrap();
}

#[test]
fn test_serialize_i128_zero() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> { Ok(()) }
        fn write_i128(&mut self, _writer: &mut impl io::Write, _value: i128) -> Result<()> { Ok(()) }
        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> { Ok(()) }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    serializer.serialize_i128(0).unwrap();
}

#[test]
fn test_serialize_i128_max() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> { Ok(()) }
        fn write_i128(&mut self, _writer: &mut impl io::Write, _value: i128) -> Result<()> { Ok(()) }
        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> { Ok(()) }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    serializer.serialize_i128(i128::MAX).unwrap();
}

#[test]
fn test_serialize_i128_negative() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> { Ok(()) }
        fn write_i128(&mut self, _writer: &mut impl io::Write, _value: i128) -> Result<()> { Ok(()) }
        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> { Ok(()) }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    serializer.serialize_i128(-1).unwrap();
}

