// Answer 0

#[test]
fn test_indent_with_single_write_error() {
    struct MockWriter {
        error_on_write: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            if self.error_on_write {
                Err(io::Error::new(io::ErrorKind::Other, "write error"))
            } else {
                Ok(0)
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { error_on_write: true };
    let result = indent(&mut writer, 1, b"test");
    // result should be an Err(io::Error)
}

#[test]
fn test_indent_with_multiple_write_errors() {
    struct MockWriter {
        error_on_write: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            if self.error_on_write {
                Err(io::Error::new(io::ErrorKind::Other, "write error"))
            } else {
                Ok(0)
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { error_on_write: true };
    let result = indent(&mut writer, 10, b"test");
    // result should be an Err(io::Error)
}

#[test]
fn test_indent_with_high_value_n() {
    struct MockWriter {
        error_on_write: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            if self.error_on_write {
                Err(io::Error::new(io::ErrorKind::Other, "write error"))
            } else {
                Ok(0)
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { error_on_write: true };
    let result = indent(&mut writer, 1000, b"test");
    // result should be an Err(io::Error)
}

