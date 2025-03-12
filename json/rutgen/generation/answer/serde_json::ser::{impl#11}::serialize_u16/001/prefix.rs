// Answer 0

#[test]
fn test_serialize_u16_io_error() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "I/O error")))
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "I/O error")))
        }
        fn write_u16(&mut self, _: &mut dyn io::Write, _: u16) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let result = serializer.serialize_u16(42);
    // No assertion, result should be Err(...).
}

#[test]
fn test_serialize_u16_io_error_boundary_min() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "I/O error")))
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "I/O error")))
        }
        fn write_u16(&mut self, _: &mut dyn io::Write, _: u16) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let result = serializer.serialize_u16(0);
    // No assertion, result should be Err(...).
}

#[test]
fn test_serialize_u16_io_error_boundary_max() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "I/O error")))
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "I/O error")))
        }
        fn write_u16(&mut self, _: &mut dyn io::Write, _: u16) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let result = serializer.serialize_u16(u16::MAX);
    // No assertion, result should be Err(...).
}

