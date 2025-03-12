// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn end_array<W: io::Write>(&mut self, _: &mut W) -> Result<()> {
            Ok(())
        }

        fn end_object_value<W: io::Write>(&mut self, _: &mut W) -> Result<()> {
            Ok(())
        }

        fn end_object<W: io::Write>(&mut self, _: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { data: Vec::new() };
    let formatter = MockFormatter;
    let state = State::Empty;

    let compound = Compound::Map { ser: &mut Serializer { writer, formatter }, state };

    let _ = compound.end();
}

#[test]
#[should_panic]
fn test_end_with_invalid_state() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn end_array<W: io::Write>(&mut self, _: &mut W) -> Result<()> {
            Ok(())
        }

        fn end_object_value<W: io::Write>(&mut self, _: &mut W) -> Result<()> {
            Ok(())
        }

        fn end_object<W: io::Write>(&mut self, _: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { data: Vec::new() };

    // Invalid state; cannot specify a state other than State::Empty
    let invalid_state = State::Rest;
    let formatter = MockFormatter;

    let compound = Compound::Map { ser: &mut Serializer { writer, formatter }, invalid_state };

    let _ = compound.end();
}

