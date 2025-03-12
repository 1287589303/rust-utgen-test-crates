// Answer 0

#[test]
fn test_serialize_f32_finite() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut TestWriter, value: f32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_f32(3.14);
}

#[test]
#[should_panic]
fn test_serialize_f32_nan() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut TestWriter, _value: f32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_f32(f32::NAN);
}

#[test]
#[should_panic]
fn test_serialize_f32_infinity() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut TestWriter, _value: f32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_f32(f32::INFINITY);
}

#[test]
fn test_serialize_f32_writer_error() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::syntax(ErrorCode::CustomError, 0, 0))
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut TestWriter, _value: f32) -> Result<()> {
            Err(Error::syntax(ErrorCode::CustomError, 0, 0))
        }

        fn end_string(&mut self, _writer: &mut TestWriter) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_f32(3.14);
}

