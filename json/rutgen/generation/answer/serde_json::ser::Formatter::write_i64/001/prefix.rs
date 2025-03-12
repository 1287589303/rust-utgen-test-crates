// Answer 0

#[test]
fn test_write_i64_negative_boundary() {
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
    let value: i64 = -i64::MAX - 1;
    let mut formatter = DummyFormatter;

    let _ = formatter.write_i64(&mut writer, value);
}

#[test]
fn test_write_i64_negative() {
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
    let value: i64 = -1;
    let mut formatter = DummyFormatter;

    let _ = formatter.write_i64(&mut writer, value);
}

#[test]
fn test_write_i64_zero() {
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
    let value: i64 = 0;
    let mut formatter = DummyFormatter;

    let _ = formatter.write_i64(&mut writer, value);
}

#[test]
fn test_write_i64_positive() {
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
    let value: i64 = 1;
    let mut formatter = DummyFormatter;

    let _ = formatter.write_i64(&mut writer, value);
}

#[test]
fn test_write_i64_positive_boundary() {
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
    let value: i64 = i64::MAX;
    let mut formatter = DummyFormatter;

    let _ = formatter.write_i64(&mut writer, value);
}

struct DummyFormatter; // A placeholder struct for testing purposes to satisfy the trait bounds.

