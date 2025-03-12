// Answer 0

#[test]
fn test_begin_array_with_valid_writer() {
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
    let mut formatter = TestFormatter;
    formatter.begin_array(&mut writer).unwrap();
}

#[test]
#[should_panic]
fn test_begin_array_with_null_writer() {
    let mut formatter = TestFormatter;
    formatter.begin_array(&mut *(std::ptr::null_mut() as *mut dyn io::Write)).unwrap();
}

#[test]
fn test_begin_array_with_empty_writer() {
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
    let mut formatter = TestFormatter;
    formatter.begin_array(&mut writer).unwrap();
}

#[test]
fn test_begin_array_with_writer_check_output() {
    struct CheckWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for CheckWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = CheckWriter { buffer: Vec::new() };
    let mut formatter = TestFormatter;
    formatter.begin_array(&mut writer).unwrap();
    assert_eq!(writer.buffer, b"[");

    // Reset buffer and call again to ensure consistent state
    writer.buffer.clear();
    formatter.begin_array(&mut writer).unwrap();
    assert_eq!(writer.buffer, b"[");
}

