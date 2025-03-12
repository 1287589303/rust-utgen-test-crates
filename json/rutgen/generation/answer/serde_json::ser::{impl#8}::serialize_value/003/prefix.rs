// Answer 0

#[test]
fn test_serialize_value_with_map() {
    struct MockWriter {
        data: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_object_value(&self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object_value(&self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { data: String::new() };
    let formatter = MockFormatter;

    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::Empty,
    };

    struct SerializableValue;

    impl Serialize for SerializableValue {
        fn serialize<S>(&self, _serializer: S) -> Result<()>
        where
            S: ser::Serializer,
        {
            Ok(())
        }
    }

    let value = SerializableValue;
    let _ = compound.serialize_value(&value);
}

#[test]
fn test_serialize_value_with_empty_map() {
    struct MockWriter {
        data: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_object_value(&self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object_value(&self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { data: String::new() };
    let formatter = MockFormatter;

    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::Empty,
    };

    struct EmptySerializableValue;

    impl Serialize for EmptySerializableValue {
        fn serialize<S>(&self, _serializer: S) -> Result<()>
        where
            S: ser::Serializer,
        {
            Ok(())
        }
    }

    let value = EmptySerializableValue;
    let _ = compound.serialize_value(&value);
}

