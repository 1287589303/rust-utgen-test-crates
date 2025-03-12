// Answer 0

#[test]
fn test_serialize_i16_min() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let mut serializer = Serializer {
        writer,
        formatter: CompactFormatter,
    };

    let result = serializer.serialize_i16(-32_768);
}

#[test]
fn test_serialize_i16_boundary_negative() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let mut serializer = Serializer {
        writer,
        formatter: CompactFormatter,
    };

    let result = serializer.serialize_i16(-1);
}

#[test]
fn test_serialize_i16_zero() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let mut serializer = Serializer {
        writer,
        formatter: CompactFormatter,
    };

    let result = serializer.serialize_i16(0);
}

#[test]
fn test_serialize_i16_boundary_positive() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let mut serializer = Serializer {
        writer,
        formatter: CompactFormatter,
    };

    let result = serializer.serialize_i16(1);
}

#[test]
fn test_serialize_i16_max() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let mut serializer = Serializer {
        writer,
        formatter: CompactFormatter,
    };

    let result = serializer.serialize_i16(32_767);
}

