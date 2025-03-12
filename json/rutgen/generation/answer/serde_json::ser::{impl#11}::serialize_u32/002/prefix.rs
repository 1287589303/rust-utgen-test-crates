// Answer 0

#[test]
fn test_serialize_u32_ok() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn write_u32(&mut self, _: &mut dyn io::Write, _: u32) -> Result<()> {
            Err(Error)
        }
        
        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_u32(0);
}

#[test]
fn test_serialize_u32_err() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn write_u32(&mut self, _: &mut dyn io::Write, _: u32) -> Result<()> {
            Err(Error)
        }
        
        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_u32(1);
}

#[test]
fn test_serialize_u32_max() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn write_u32(&mut self, _: &mut dyn io::Write, _: u32) -> Result<()> {
            Err(Error)
        }
        
        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_u32(u32::MAX);
}

#[test]
fn test_serialize_u32_invalid() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn write_u32(&mut self, _: &mut dyn io::Write, _: u32) -> Result<()> {
            Err(Error)
        }
        
        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_u32(42);
}

