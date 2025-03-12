// Answer 0

#[test]
fn test_write_char_escape_carriage_return() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let mut formatter = TestFormatter {};

    let char_escape = CharEscape::CarriageReturn;
    let _ = formatter.write_char_escape(&mut writer, char_escape);
}

struct TestFormatter; 

impl Formatter for TestFormatter {}

