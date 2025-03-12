// Answer 0

#[test]
fn test_end_function_with_non_empty_state() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let mut ser = Serializer { writer, formatter };

    let state = State::First; // Not State::Empty
    let compound = Compound::Map { ser: &mut ser, state };

    compound.end().expect("Failed to call end");
}

