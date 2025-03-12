// Answer 0

#[test]
fn test_collect_str_with_string() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {}

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    let value = "test string";
    let serializer = &mut Serializer {
        writer: writer,
        formatter: formatter,
    };

    let _ = serializer.collect_str(&value);
}

#[test]
fn test_collect_str_with_integer() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {}

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    let value = 1234;
    let serializer = &mut Serializer {
        writer: writer,
        formatter: formatter,
    };

    let _ = serializer.collect_str(&value);
}

#[test]
fn test_collect_str_with_floating_point() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {}

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    let value = 3.14159;
    let serializer = &mut Serializer {
        writer: writer,
        formatter: formatter,
    };

    let _ = serializer.collect_str(&value);
}

#[test]
fn test_collect_str_with_custom_display() {
    struct CustomDisplay;

    impl Display for CustomDisplay {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Custom display string")
        }
    }

    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {}

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    let value = CustomDisplay;
    let serializer = &mut Serializer {
        writer: writer,
        formatter: formatter,
    };

    let _ = serializer.collect_str(&value);
}

