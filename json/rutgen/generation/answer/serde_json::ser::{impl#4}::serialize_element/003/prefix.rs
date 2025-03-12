// Answer 0

#[test]
fn test_serialize_element_first() {
    struct TestWriter;
    
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_array_value(&mut self, _writer: &mut impl io::Write, _first: bool) -> Result<()> {
            Ok(())
        }
        
        fn end_array_value(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::First,
    };
    
    let value = 42; // type that implements Serialize
    
    let _ = compound.serialize_element(&value);
}

#[test]
fn test_serialize_element_subsequent() {
    struct TestWriter;
    
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_array_value(&mut self, _writer: &mut impl io::Write, _first: bool) -> Result<()> {
            Ok(())
        }
        
        fn end_array_value(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::Rest,
    };
    
    let value = 42; // type that implements Serialize
    
    let _ = compound.serialize_element(&value);
}

