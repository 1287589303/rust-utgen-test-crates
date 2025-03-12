// Answer 0

#[test]
#[should_panic]
fn test_serialize_seq_success_with_zero_length() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(_buf.len()) }
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn begin_array(&mut self, _writer: &mut impl io::Write) -> Result<()> { Ok(()) }
        fn end_array(&mut self, _writer: &mut impl io::Write) -> Result<()> { Err(Error) }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let ser = Serializer { writer, formatter };
    let result = ser.serialize_seq(Some(0));
}

#[test]
#[should_panic]
fn test_serialize_seq_failure_on_end_array() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(_buf.len()) }
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn begin_array(&mut self, _writer: &mut impl io::Write) -> Result<()> { Ok(()) }
        fn end_array(&mut self, _writer: &mut impl io::Write) -> Result<()> { Err(Error) }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let ser = Serializer { writer, formatter };
    let result = ser.serialize_seq(Some(0));
}

