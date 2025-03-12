// Answer 0

#[test]
fn test_write_i8_negative_boundary() {
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
    let value: i8 = -128;
    let formatter = ();
    formatter.write_i8(&mut writer, value).unwrap();
}

#[test]
fn test_write_i8_positive_boundary() {
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
    let value: i8 = 127;
    let formatter = ();
    formatter.write_i8(&mut writer, value).unwrap();
}

#[test]
fn test_write_i8_negative() {
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
    let value: i8 = -42;
    let formatter = ();
    formatter.write_i8(&mut writer, value).unwrap();
}

#[test]
fn test_write_i8_positive() {
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
    let value: i8 = 42;
    let formatter = ();
    formatter.write_i8(&mut writer, value).unwrap();
}

#[test]
#[should_panic]
fn test_write_i8_invalid_positive() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let value: i8 = 128; // Out of bounds
    let formatter = ();
    formatter.write_i8(&mut writer, value).unwrap();
}

#[test]
#[should_panic]
fn test_write_i8_invalid_negative() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let value: i8 = -129; // Out of bounds
    let formatter = ();
    formatter.write_i8(&mut writer, value).unwrap();
}

