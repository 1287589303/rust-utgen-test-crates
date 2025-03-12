// Answer 0

#[test]
fn test_end_array_with_valid_writer() {
    struct TestWriter {
        buf: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buf.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { buf: Vec::new() };
    let mut formatter = |_: &mut dyn io::Write| -> io::Result<()> { Ok(()) };
    formatter.end_array(&mut writer).unwrap();
}

#[test]
fn test_end_array_with_empty_writer() {
    struct EmptyWriter;

    impl io::Write for EmptyWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = EmptyWriter;
    let mut formatter = |_: &mut dyn io::Write| -> io::Result<()> { Ok(()) };
    formatter.end_array(&mut writer).unwrap();
}

#[test]
fn test_end_array_with_error_writer() {
    struct ErrorWriter;

    impl io::Write for ErrorWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = ErrorWriter;
    let mut formatter = |_: &mut dyn io::Write| -> io::Result<()> { Ok(()) };
    let result = formatter.end_array(&mut writer);
    assert!(result.is_err());
}

#[test]
fn test_end_array_with_large_writer() {
    struct LargeWriter {
        buf: Vec<u8>,
    }

    impl io::Write for LargeWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buf.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = LargeWriter { buf: Vec::new() };
    let mut formatter = |_: &mut dyn io::Write| -> io::Result<()> { Ok(()) };
    formatter.end_array(&mut writer).unwrap();
    assert_eq!(writer.buf, b"]");
}

