// Answer 0

#[test]
fn test_serialize_f64_nan() {
    struct MockWriter;
    struct MockFormatter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = map_key_serializer.serialize_f64(f64::NAN);
}

#[test]
fn test_serialize_f64_infinity() {
    struct MockWriter;
    struct MockFormatter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = map_key_serializer.serialize_f64(f64::INFINITY);
}

#[test]
fn test_serialize_f64_negative_infinity() {
    struct MockWriter;
    struct MockFormatter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = map_key_serializer.serialize_f64(f64::NEG_INFINITY);
}

