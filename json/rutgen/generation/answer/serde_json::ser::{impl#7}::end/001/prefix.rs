// Answer 0

#[test]
fn test_end_with_state_first_and_error() {
    struct FakeWriter;
    
    impl io::Write for FakeWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(0)
        }
        
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }
    
    struct FakeFormatter;

    impl Formatter for FakeFormatter {
        fn end_array(&self, _writer: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "forced error"))
        }
        
        fn end_object_value(&self, _writer: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object(&self, _writer: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    let mut writer = FakeWriter;
    let formatter = FakeFormatter;
    
    let mut compound = Compound::Map {
        ser: &mut Serializer {
            writer,
            formatter,
        },
        state: State::First,
    };
    
    let result = compound.end();
}

#[test]
fn test_end_with_state_rest_and_error() {
    struct FakeWriter;
    
    impl io::Write for FakeWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(0)
        }
        
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }
    
    struct FakeFormatter;

    impl Formatter for FakeFormatter {
        fn end_array(&self, _writer: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "forced error"))
        }
        
        fn end_object_value(&self, _writer: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object(&self, _writer: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    let mut writer = FakeWriter;
    let formatter = FakeFormatter;
    
    let mut compound = Compound::Map {
        ser: &mut Serializer {
            writer,
            formatter,
        },
        state: State::Rest,
    };
    
    let result = compound.end();
}

