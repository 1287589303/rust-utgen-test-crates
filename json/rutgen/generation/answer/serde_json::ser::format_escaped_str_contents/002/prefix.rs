// Answer 0

#[test]
fn test_format_escaped_str_contents_with_escape_chars() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, _: &mut dyn io::Write, _: &str) -> io::Result<()> {
            Ok(())
        }
        
        fn write_char_escape(&mut self, _: &mut dyn io::Write, _: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;

    let value = "abc\n\tdef\b\fghi\r\""; // contains newline, tab, backspace, form feed, carriage return, and quote
    let _ = format_escaped_str_contents(&mut writer, &mut formatter, value);
}

#[test]
fn test_format_escaped_str_contents_with_only_unescaped_chars() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, _: &mut dyn io::Write, _: &str) -> io::Result<()> {
            Ok(())
        }
        
        fn write_char_escape(&mut self, _: &mut dyn io::Write, _: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;

    let value = "abcdef"; // only unescaped characters
    let _ = format_escaped_str_contents(&mut writer, &mut formatter, value);
}

#[test]
fn test_format_escaped_str_contents_with_multiple_escape_chars() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, _: &mut dyn io::Write, _: &str) -> io::Result<()> {
            Ok(())
        }
        
        fn write_char_escape(&mut self, _: &mut dyn io::Write, _: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;

    let value = "\tThis is a test string with multiple escapes:\n\t\b\fHello, world!\r\"End\""; // contains multiple escape sequences
    let _ = format_escaped_str_contents(&mut writer, &mut formatter, value);
}

