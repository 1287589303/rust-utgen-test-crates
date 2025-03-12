// Answer 0

#[test]
fn test_write_u64_zero() {
    struct MockWriter(Vec<u8>);

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter(Vec::new());
    let mut formatter = MockFormatter;

    let result = formatter.write_u64(&mut writer, 0);
}

#[test]
fn test_write_u64_middle() {
    struct MockWriter(Vec<u8>);

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter(Vec::new());
    let mut formatter = MockFormatter;

    let result = formatter.write_u64(&mut writer, 123456789);
}

#[test]
fn test_write_u64_max() {
    struct MockWriter(Vec<u8>);

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter(Vec::new());
    let mut formatter = MockFormatter;

    let result = formatter.write_u64(&mut writer, 18446744073709551615);
}

#[test]
fn test_write_u64_mid_boundary() {
    struct MockWriter(Vec<u8>);

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter(Vec::new());
    let mut formatter = MockFormatter;

    let result = formatter.write_u64(&mut writer, 9223372036854775808);
}

