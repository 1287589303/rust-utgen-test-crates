// Answer 0

#[test]
fn test_serialize_i32_ok_then_err() {
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

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i32(&mut self, _writer: &mut impl io::Write, _value: i32) -> Result<()> {
            Err(Error)
        }

        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };
    let value = 0;  // any valid i32 value in the range
    let _result = serializer.serialize_i32(value);
}

#[test]
fn test_serialize_i32_bound_negative() {
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

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i32(&mut self, _writer: &mut impl io::Write, _value: i32) -> Result<()> {
            Err(Error)
        }

        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };
    let value = -2147483648;  // boundary value
    let _result = serializer.serialize_i32(value);
}

#[test]
fn test_serialize_i32_bound_positive() {
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

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i32(&mut self, _writer: &mut impl io::Write, _value: i32) -> Result<()> {
            Err(Error)
        }

        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };
    let value = 2147483647;  // boundary value
    let _result = serializer.serialize_i32(value);
}

