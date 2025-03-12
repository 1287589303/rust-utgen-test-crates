// Answer 0

#[test]
#[should_panic]
fn test_serialize_bool_with_null_writer() {
    struct MalformedWriter;
    let formatter = CompactFormatter;
    let serializer = Serializer {
        writer: MalformedWriter,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    let _ = map_key_serializer.serialize_bool(true);
}

#[test]
#[should_panic]
fn test_serialize_bool_with_invalid_formatter() {
    struct InvalidFormatter;
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let serializer = Serializer {
        writer,
        formatter: InvalidFormatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    let _ = map_key_serializer.serialize_bool(false);
}

#[test]
#[should_panic]
fn test_serialize_bool_with_memory_error() {
    struct MemoryErrorWriter;

    impl io::Write for MemoryErrorWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::from(ErrorCode::MemoryError))
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = MemoryErrorWriter;
    let formatter = CompactFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    let _ = map_key_serializer.serialize_bool(true);
}

