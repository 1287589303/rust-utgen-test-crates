// Answer 0

#[test]
fn test_end_empty_state() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = CompactFormatter;
    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::Empty,
    };

    compound.end();
}

#[test]
fn test_end_first_state() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = CompactFormatter;
    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::First,
    };

    compound.end();
}

#[test]
fn test_end_rest_state() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = CompactFormatter;
    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::Rest,
    };

    compound.end();
}

