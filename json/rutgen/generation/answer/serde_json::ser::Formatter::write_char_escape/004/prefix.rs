// Answer 0

#[test]
fn test_write_char_escape_line_feed() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter { output: Vec::new() };
    let char_escape = CharEscape::LineFeed;
    writer.write_char_escape(&mut writer, char_escape).unwrap();
}

#[test]
fn test_write_char_escape_quote() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter { output: Vec::new() };
    let char_escape = CharEscape::Quote;
    writer.write_char_escape(&mut writer, char_escape).unwrap();
}

#[test]
fn test_write_char_escape_reverse_solidus() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter { output: Vec::new() };
    let char_escape = CharEscape::ReverseSolidus;
    writer.write_char_escape(&mut writer, char_escape).unwrap();
}

#[test]
fn test_write_char_escape_solidus() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter { output: Vec::new() };
    let char_escape = CharEscape::Solidus;
    writer.write_char_escape(&mut writer, char_escape).unwrap();
}

#[test]
fn test_write_char_escape_backspace() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter { output: Vec::new() };
    let char_escape = CharEscape::Backspace;
    writer.write_char_escape(&mut writer, char_escape).unwrap();
}

#[test]
fn test_write_char_escape_form_feed() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter { output: Vec::new() };
    let char_escape = CharEscape::FormFeed;
    writer.write_char_escape(&mut writer, char_escape).unwrap();
}

#[test]
fn test_write_char_escape_carriage_return() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter { output: Vec::new() };
    let char_escape = CharEscape::CarriageReturn;
    writer.write_char_escape(&mut writer, char_escape).unwrap();
}

#[test]
fn test_write_char_escape_tab() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter { output: Vec::new() };
    let char_escape = CharEscape::Tab;
    writer.write_char_escape(&mut writer, char_escape).unwrap();
}

#[test]
fn test_write_char_escape_ascii_control() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter { output: Vec::new() };
    for byte in 0..=255 {
        let char_escape = CharEscape::AsciiControl(byte);
        writer.write_char_escape(&mut writer, char_escape).unwrap();
    }
}

