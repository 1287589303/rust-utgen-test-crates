// Answer 0

#[test]
fn test_serialize_u16_zero() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    let mut writer = DummyWriter;
    let formatter = CompactFormatter; 
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_u16(0);
}

#[test]
fn test_serialize_u16_one() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    let mut writer = DummyWriter;
    let formatter = CompactFormatter; 
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_u16(1);
}

#[test]
fn test_serialize_u16_half_max() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    let mut writer = DummyWriter;
    let formatter = CompactFormatter; 
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_u16(32767);
}

#[test]
fn test_serialize_u16_max() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    let mut writer = DummyWriter;
    let formatter = CompactFormatter; 
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_u16(65535);
}

#[should_panic]
fn test_serialize_u16_overflow() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    let mut writer = DummyWriter;
    let formatter = CompactFormatter; 
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_u16(65536);
}

#[should_panic]
fn test_serialize_u16_negative() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    let mut writer = DummyWriter;
    let formatter = CompactFormatter; 
    let serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_u16(u16::from(-1));
}

