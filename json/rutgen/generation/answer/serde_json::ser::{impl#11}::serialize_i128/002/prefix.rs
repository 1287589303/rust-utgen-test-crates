// Answer 0

#[test]
fn test_serialize_i128_boundary_negative() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(0)
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct DummyFormatter;
    impl Formatter for DummyFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i128(&mut self, _writer: &mut impl io::Write, _value: i128) -> Result<()> {
            Err(Error)
        }

        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let formatter = DummyFormatter;
    let serializer = Serializer { writer, formatter };

    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_i128(-2_i128.pow(127));
}

#[test]
fn test_serialize_i128_boundary_positive() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(0)
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct DummyFormatter;
    impl Formatter for DummyFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i128(&mut self, _writer: &mut impl io::Write, _value: i128) -> Result<()> {
            Err(Error)
        }

        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let formatter = DummyFormatter;
    let serializer = Serializer { writer, formatter };

    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_i128(2_i128.pow(127) - 1);
}

