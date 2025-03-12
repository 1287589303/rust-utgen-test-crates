// Answer 0

#[test]
fn test_format_escaped_str_empty() {
    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> io::Result<()> {
            Ok(())
        }

        fn write_string_fragment(&mut self, _: &mut dyn io::Write, _: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape(&mut self, _: &mut dyn io::Write, _: CharEscape) -> io::Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn io::Write) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut formatter = TestFormatter;
    let value = "";

    let _ = format_escaped_str(&mut writer, &mut formatter, value);
}

#[test]
fn test_format_escaped_str_with_escape_sequences() {
    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> io::Result<()> {
            Ok(())
        }

        fn write_string_fragment(&mut self, _: &mut dyn io::Write, _: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape(&mut self, _: &mut dyn io::Write, _: CharEscape) -> io::Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn io::Write) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut formatter = TestFormatter;
    let value = r#"This is a test string with escape characters: \" \n \t \r"#;

    let _ = format_escaped_str(&mut writer, &mut formatter, value);
}

#[test]
fn test_format_escaped_str_long_string() {
    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> io::Result<()> {
            Ok(())
        }

        fn write_string_fragment(&mut self, _: &mut dyn io::Write, _: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape(&mut self, _: &mut dyn io::Write, _: CharEscape) -> io::Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn io::Write) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut formatter = TestFormatter;
    let value = "A".repeat(256); // 256 characters long

    let _ = format_escaped_str(&mut writer, &mut formatter, &value);
}

