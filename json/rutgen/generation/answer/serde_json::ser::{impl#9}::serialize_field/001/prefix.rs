// Answer 0

#[test]
fn test_serialize_field_map() {
    struct TestWriter;
    struct TestFormatter;

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let mut compound = Compound::Map {
        ser: &mut Serializer {
            writer,
            formatter,
        },
        state: State::Empty,
    };
    let key: &'static str = "test_key";
    let value = "test_value";

    let _ = compound.serialize_field(key, &value);
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_serialize_field_number() {
    struct TestWriter;
    struct TestFormatter;

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let mut compound = Compound::Number {
        ser: &mut Serializer {
            writer,
            formatter,
        },
    };
    let key: &'static str = crate::number::TOKEN;
    let value = 42;

    let _ = compound.serialize_field(key, &value);
}

#[test]
#[cfg(feature = "raw_value")]
fn test_serialize_field_raw_value() {
    struct TestWriter;
    struct TestFormatter;

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let mut compound = Compound::RawValue {
        ser: &mut Serializer {
            writer,
            formatter,
        },
    };
    let key: &'static str = crate::raw::TOKEN;
    let value = "raw_value";

    let _ = compound.serialize_field(key, &value);
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_serialize_field_number_invalid_key() {
    struct TestWriter;
    struct TestFormatter;

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let mut compound = Compound::Number {
        ser: &mut Serializer {
            writer,
            formatter,
        },
    };
    let key: &'static str = "invalid_key";
    let value = 42;

    let _ = compound.serialize_field(key, &value);
}

#[test]
#[cfg(feature = "raw_value")]
fn test_serialize_field_raw_value_invalid_key() {
    struct TestWriter;
    struct TestFormatter;

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let mut compound = Compound::RawValue {
        ser: &mut Serializer {
            writer,
            formatter,
        },
    };
    let key: &'static str = "invalid_key";
    let value = "raw_value";

    let _ = compound.serialize_field(key, &value);
}

