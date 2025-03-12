// Answer 0

#[test]
fn test_serialize_unit_struct_valid() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(_buf.len()) }
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    
    let writer = MockWriter;
    let serializer = Serializer { writer, formatter: CompactFormatter {} };
    let name = "ValidStruct";
    
    let _ = serializer.serialize_unit_struct(name);
}

#[test]
#[should_panic]
fn test_serialize_unit_struct_invalid() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Err(Error::new(ErrorCode::IoError)) }
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> { Err(Error::new(ErrorCode::IoError)) }
        fn flush(&mut self) -> Result<()> { Err(Error::new(ErrorCode::IoError)) }
    }
    
    let writer = MockWriter;
    let serializer = Serializer { writer, formatter: CompactFormatter {} };
    let name = "InvalidStruct";
    
    let _ = serializer.serialize_unit_struct(name);
}

#[test]
fn test_serialize_unit_struct_empty_name() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(_buf.len()) }
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    
    let writer = MockWriter;
    let serializer = Serializer { writer, formatter: CompactFormatter {} };
    let empty_name: &'static str = "";
    
    let _ = serializer.serialize_unit_struct(empty_name);
}

