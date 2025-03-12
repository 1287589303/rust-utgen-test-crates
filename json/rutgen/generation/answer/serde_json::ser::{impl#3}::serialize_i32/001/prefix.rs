// Answer 0

#[test]
fn test_serialize_min_i32() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_i32(-2147483648);
}

#[test]
fn test_serialize_zero_i32() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_i32(0);
}

#[test]
fn test_serialize_positive_i32() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_i32(2147483647);
}

#[test]
fn test_serialize_negative_i32() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_i32(-1);
}

#[test]
fn test_serialize_edge_positive() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_i32(1);
}

#[test]
fn test_serialize_edge_negative() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_i32(-2147483647);
}

