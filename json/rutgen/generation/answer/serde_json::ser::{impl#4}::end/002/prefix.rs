// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn end_array(&self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }
    
    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let compound = Compound::Map { ser: &mut serializer, state: State::Empty };
    let result = compound.end();
}

#[test]
fn test_end_with_non_empty_state() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn end_array(&self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }
    
    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let compound = Compound::Map { ser: &mut serializer, state: State::Rest };
    let result = compound.end();
}

