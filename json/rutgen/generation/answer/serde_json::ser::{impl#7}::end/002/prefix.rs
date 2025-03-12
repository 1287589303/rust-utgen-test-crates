// Answer 0

#[test]
fn test_end_with_state_rest_and_formatter_ok_and_object_value_err() {
    struct TestWriter {
        data: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn end_array(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object_value(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::from(ErrorCode::CustomError))
        }

        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { data: Vec::new() };
    let formatter = TestFormatter;
    let state = State::Rest;

    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state,
    };

    let result = compound.end();
}

#[test]
fn test_end_with_state_rest_and_formatter_ok_and_object_value_ok() {
    struct TestWriter {
        data: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn end_array(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object_value(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { data: Vec::new() };
    let formatter = TestFormatter;
    let state = State::Rest;

    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state,
    };

    let result = compound.end();
}

#[test]
#[should_panic]
fn test_end_with_state_rest_and_formatter_err_and_object_value_ok() {
    struct TestWriter {
        data: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn end_array(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::from(ErrorCode::CustomError))
        }

        fn end_object_value(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { data: Vec::new() };
    let formatter = TestFormatter;
    let state = State::Rest;

    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state,
    };

    let _result = compound.end();
}

