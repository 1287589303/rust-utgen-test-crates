// Answer 0

#[test]
#[should_panic]
fn test_format_escaped_str_formatter_begin_string_err() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Err(Error::new(ErrorCode::Other))
        }

        fn end_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn write_string_fragment<W: io::Write>(&mut self, _writer: &mut W, _fragment: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, _writer: &mut W, _escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let mut formatter = TestFormatter;
    let value = r#"{"key": "value with \"escaped\" characters"}"#;

    format_escaped_str(&mut writer, &mut formatter, value).unwrap();
}

