// Answer 0

#[test]
fn test_serialize_u8_zero() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct MockFormatter;
    impl MockFormatter {
        fn write_u8(&mut self, _writer: &mut MockWriter, _value: u8) -> Result<usize> { Ok(1) }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = &mut Serializer { writer, formatter };
    let result = serializer.serialize_u8(0);
}

#[test]
fn test_serialize_u8_boundary_min() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct MockFormatter;
    impl MockFormatter {
        fn write_u8(&mut self, _writer: &mut MockWriter, _value: u8) -> Result<usize> { Ok(1) }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = &mut Serializer { writer, formatter };
    let result = serializer.serialize_u8(1);
}

#[test]
fn test_serialize_u8_boundary_max() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct MockFormatter;
    impl MockFormatter {
        fn write_u8(&mut self, _writer: &mut MockWriter, _value: u8) -> Result<usize> { Ok(1) }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = &mut Serializer { writer, formatter };
    let result = serializer.serialize_u8(255);
}

#[test]
fn test_serialize_u8_middle_value() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct MockFormatter;
    impl MockFormatter {
        fn write_u8(&mut self, _writer: &mut MockWriter, _value: u8) -> Result<usize> { Ok(1) }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = &mut Serializer { writer, formatter };
    let result = serializer.serialize_u8(128);
}

