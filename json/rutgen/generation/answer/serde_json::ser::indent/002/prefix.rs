// Answer 0

#[test]
fn test_indent_with_zero_n() {
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
    let result = indent(&mut writer, 0, b"test");
}

#[test]
fn test_indent_with_positive_n() {
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
    let result = indent(&mut writer, 3, b"test");
}

#[test]
#[should_panic]
fn test_indent_with_invalid_n() {
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
    let result = indent(&mut writer, usize::MAX, b"test"); // Out of range
}

