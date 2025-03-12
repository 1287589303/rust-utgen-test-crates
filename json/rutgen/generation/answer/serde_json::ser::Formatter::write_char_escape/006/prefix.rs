// Answer 0

#[test]
fn test_write_char_escape_backspace() {
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

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut formatter = FormatterImpl; // assume FormatterImpl implements Formatter

    let char_escape = CharEscape::Backspace;

    let _ = formatter.write_char_escape(&mut writer, char_escape);
}

