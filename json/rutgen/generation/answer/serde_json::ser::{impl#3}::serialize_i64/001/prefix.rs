// Answer 0

#[test]
fn test_serialize_i64_min() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_i64(&mut self, writer: &mut MockWriter, value: i64) -> Result<()> {
            // Mock writing behavior
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_i64(i64::MIN);
}

#[test]
fn test_serialize_i64_negative_one() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_i64(&mut self, writer: &mut MockWriter, value: i64) -> Result<()> {
            // Mock writing behavior
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_i64(-1);
}

#[test]
fn test_serialize_i64_zero() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_i64(&mut self, writer: &mut MockWriter, value: i64) -> Result<()> {
            // Mock writing behavior
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_i64(0);
}

#[test]
fn test_serialize_i64_positive_one() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_i64(&mut self, writer: &mut MockWriter, value: i64) -> Result<()> {
            // Mock writing behavior
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_i64(1);
}

#[test]
fn test_serialize_i64_max() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_i64(&mut self, writer: &mut MockWriter, value: i64) -> Result<()> {
            // Mock writing behavior
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_i64(i64::MAX);
}

