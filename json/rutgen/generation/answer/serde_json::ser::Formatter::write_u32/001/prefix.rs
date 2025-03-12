// Answer 0

#[test]
fn test_write_u32_min_value() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let value: u32 = 0;
    let mut formatter = DummyFormatter;

    let _ = formatter.write_u32(&mut writer, value);
}

#[test]
fn test_write_u32_mid_value() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let value: u32 = 2147483648; // mid-range value
    let mut formatter = DummyFormatter;

    let _ = formatter.write_u32(&mut writer, value);
}

#[test]
fn test_write_u32_max_value() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let value: u32 = 4294967295; // max value
    let mut formatter = DummyFormatter;

    let _ = formatter.write_u32(&mut writer, value);
}

#[test]
fn test_write_u32_zero_to_max() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let values = [0, 1, 2, 4294967294, 4294967295];
    let mut formatter = DummyFormatter;

    for &value in &values {
        let _ = formatter.write_u32(&mut writer, value);
    }
}

