// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct TestFormatter;
    
    impl Formatter for TestFormatter {
        fn begin_string<W>(&mut self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
        
        fn write_bool<W>(&mut self, _writer: &mut W, _value: bool) -> Result<()> {
            Ok(())
        }
        
        fn end_string<W>(&mut self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }
    
    struct TestWriter;
    
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let mut formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_bool(true);
}

#[test]
fn test_serialize_bool_false() {
    struct TestFormatter;
    
    impl Formatter for TestFormatter {
        fn begin_string<W>(&mut self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
        
        fn write_bool<W>(&mut self, _writer: &mut W, _value: bool) -> Result<()> {
            Ok(())
        }
        
        fn end_string<W>(&mut self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }
    
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let mut formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_bool(false);
}

