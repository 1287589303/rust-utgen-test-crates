// Answer 0

#[test]
fn test_end_with_state_first() {
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

    let writer = MockWriter;
    let formatter = MockFormatter;
    let state = State::First;

    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state,
    };

    let _ = compound.end();
}

#[test]
fn test_end_with_state_rest() {
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

    let writer = MockWriter;
    let formatter = MockFormatter;
    let state = State::Rest;

    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state,
    };

    let _ = compound.end();
}

