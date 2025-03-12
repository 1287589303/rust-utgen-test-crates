// Answer 0

#[test]
fn test_serialize_element_success() {
    struct MockWriter;
    struct MockFormatter;

    impl MockFormatter {
        fn begin_array_value<W>(&mut self, _writer: &mut W, _is_first: bool) -> Result<()> {
            Ok(())
        }
        fn end_array_value<W>(&mut self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let mut state = State::First;
    
    let mut comp = Compound::Map { 
        ser: &mut Serializer { writer: writer, formatter: formatter }, 
        state: state 
    };

    let value = Some(&"test");

    comp.serialize_element(&value).unwrap();
}

#[test]
fn test_serialize_element_error() {
    struct MockWriter;
    struct MockFormatter;

    impl MockFormatter {
        fn begin_array_value<W>(&mut self, _writer: &mut W, _is_first: bool) -> Result<()> {
            Ok(())
        }
        fn end_array_value<W>(&mut self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let mut state = State::First;
    
    let mut comp = Compound::Map { 
        ser: &mut Serializer { writer: writer, formatter: formatter }, 
        state: state 
    };

    struct FailingSerialize;

    impl Serialize for FailingSerialize {
        fn serialize<S>(&self, _serializer: S) -> Result<()>
        where S: ser::Serializer {
            Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "failed")))
        }
    }

    let value = FailingSerialize;

    let result = comp.serialize_element(&value);
    assert!(result.is_err());
}

