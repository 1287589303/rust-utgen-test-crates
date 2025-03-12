// Answer 0

#[test]
fn test_collect_str_with_normal_string() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let value = "Hello, World!";
    let serializer = &mut Serializer {
        writer,
        formatter,
    };

    let _ = serializer.collect_str(&value);
}

#[test]
#[should_panic]
fn test_collect_str_with_formatting_error() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let value = "This will cause formatting error";
    let serializer = &mut Serializer {
        writer,
        formatter,
    };

    let result = serializer.collect_str(&value);
    assert!(result.is_err());
}

#[test]
fn test_collect_str_with_empty_string() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let value = "";
    let serializer = &mut Serializer {
        writer,
        formatter,
    };

    let _ = serializer.collect_str(&value);
}

#[test]
fn test_collect_str_with_special_characters() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let value = "Special chars: \n\t\\\"";
    let serializer = &mut Serializer {
        writer,
        formatter,
    };

    let _ = serializer.collect_str(&value);
}

