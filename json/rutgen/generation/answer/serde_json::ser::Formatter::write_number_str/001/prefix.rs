// Answer 0

#[test]
fn test_write_number_str_positive_number() {
    struct MockWriter {
        output: Vec<u8>,
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let value = "123.456";
    // Call the function
    write_number_str(&mut writer, value).unwrap();
}

#[test]
fn test_write_number_str_negative_number() {
    struct MockWriter {
        output: Vec<u8>,
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let value = "-123.456";
    // Call the function
    write_number_str(&mut writer, value).unwrap();
}

#[test]
fn test_write_number_str_zero() {
    struct MockWriter {
        output: Vec<u8>,
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let value = "0";
    // Call the function
    write_number_str(&mut writer, value).unwrap();
}

#[test]
fn test_write_number_str_empty_string() {
    struct MockWriter {
        output: Vec<u8>,
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let value = "";
    // Call the function
    write_number_str(&mut writer, value).unwrap();
}

#[test]
#[should_panic]
fn test_write_number_str_nan() {
    struct MockWriter {
        output: Vec<u8>,
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let value = "NaN";
    // Call the function
    write_number_str(&mut writer, value).unwrap();
}

#[test]
#[should_panic]
fn test_write_number_str_infinity() {
    struct MockWriter {
        output: Vec<u8>,
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let value = "Infinity";
    // Call the function
    write_number_str(&mut writer, value).unwrap();
}

#[test]
#[should_panic]
fn test_write_number_str_negative_infinity() {
    struct MockWriter {
        output: Vec<u8>,
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let value = "-Infinity";
    // Call the function
    write_number_str(&mut writer, value).unwrap();
}

#[test]
fn test_write_number_str_large_number() {
    struct MockWriter {
        output: Vec<u8>,
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let value = "1.7976931348623157E+308"; // Maximum f64 value
    // Call the function
    write_number_str(&mut writer, value).unwrap();
}

