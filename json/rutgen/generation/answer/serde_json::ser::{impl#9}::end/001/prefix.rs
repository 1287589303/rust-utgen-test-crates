// Answer 0

#[test]
fn test_end_map_variant() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let serializer = Serializer { writer, formatter: Default::default() };
    let state = State::Empty;
    let compound = Compound::Map { ser: &mut serializer, state };
    compound.end().unwrap();
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_end_number_variant() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let serializer = Serializer { writer, formatter: Default::default() };
    let compound = Compound::Number { ser: &mut serializer };
    compound.end().unwrap();
}

#[test]
#[cfg(feature = "raw_value")]
fn test_end_raw_value_variant() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let serializer = Serializer { writer, formatter: Default::default() };
    let compound = Compound::RawValue { ser: &mut serializer };
    compound.end().unwrap();
}

