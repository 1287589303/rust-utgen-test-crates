// Answer 0

#[test]
fn test_serialize_i128_min() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(_buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let mut serializer = Serializer { writer, formatter: CompactFormatter };
    let _ = serializer.serialize_i128(i128::MIN);
}

#[test]
fn test_serialize_i128_negative() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(_buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let mut serializer = Serializer { writer, formatter: CompactFormatter };
    let _ = serializer.serialize_i128(-1);
}

#[test]
fn test_serialize_i128_zero() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(_buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let mut serializer = Serializer { writer, formatter: CompactFormatter };
    let _ = serializer.serialize_i128(0);
}

#[test]
fn test_serialize_i128_positive() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(_buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let mut serializer = Serializer { writer, formatter: CompactFormatter };
    let _ = serializer.serialize_i128(1);
}

#[test]
fn test_serialize_i128_max() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(_buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let mut serializer = Serializer { writer, formatter: CompactFormatter };
    let _ = serializer.serialize_i128(i128::MAX);
}

