// Answer 0

#[test]
#[should_panic]
fn test_serialize_f32_nan() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0))
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0))
        }
        fn write_f32(&mut self, _writer: &mut dyn io::Write, _value: f32) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }
    
    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    let _ = serializer.serialize_f32(f32::NAN);
}

#[test]
#[should_panic]
fn test_serialize_f32_infinity() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0))
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0))
        }
        fn write_f32(&mut self, _writer: &mut dyn io::Write, _value: f32) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    let _ = serializer.serialize_f32(f32::INFINITY);
}

#[test]
#[should_panic]
fn test_serialize_f32_negative_infinity() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0))
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0))
        }
        fn write_f32(&mut self, _writer: &mut dyn io::Write, _value: f32) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    let _ = serializer.serialize_f32(f32::NEG_INFINITY);
}

