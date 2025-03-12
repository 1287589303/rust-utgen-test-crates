// Answer 0

#[test]
fn test_serialize_u32_zero() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
  
    struct MockFormatter;
    impl MockFormatter {
        fn write_u32(&mut self, _writer: &mut MockWriter, _value: u32) -> Result<()> {
            Ok(())
        }
    }
  
    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_u32(0).unwrap();
}

#[test]
fn test_serialize_u32_middle() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
  
    struct MockFormatter;
    impl MockFormatter {
        fn write_u32(&mut self, _writer: &mut MockWriter, _value: u32) -> Result<()> {
            Ok(())
        }
    }
  
    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_u32(2147483648).unwrap();
}

#[test]
fn test_serialize_u32_max() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct MockFormatter;
    impl MockFormatter {
        fn write_u32(&mut self, _writer: &mut MockWriter, _value: u32) -> Result<()> {
            Ok(())
        }
    }
  
    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_u32(4294967295).unwrap();
}

