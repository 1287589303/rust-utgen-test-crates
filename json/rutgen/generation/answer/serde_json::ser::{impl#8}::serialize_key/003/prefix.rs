// Answer 0

#[test]
fn test_serialize_key_string() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let mut serializer = Serializer { writer, formatter: CompactFormatter {} };
    let mut compound = Compound::Map { ser: &mut serializer, state: State::Empty };

    let key = "test_key";
    compound.serialize_key(&key).unwrap();
}

#[test]
fn test_serialize_key_integer() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let mut serializer = Serializer { writer, formatter: CompactFormatter {} };
    let mut compound = Compound::Map { ser: &mut serializer, state: State::Empty };

    let key = 42;
    compound.serialize_key(&key).unwrap();
}

#[test]
fn test_serialize_key_empty_collection() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let mut serializer = Serializer { writer, formatter: CompactFormatter {} };
    let mut compound = Compound::Map { ser: &mut serializer, state: State::Empty };

    let key: Vec<u8> = vec![];
    compound.serialize_key(&key).unwrap();
}

#[test]
fn test_serialize_key_special_characters() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let mut serializer = Serializer { writer, formatter: CompactFormatter {} };
    let mut compound = Compound::Map { ser: &mut serializer, state: State::Empty };

    let key = "key_with_special_chars!@#";
    compound.serialize_key(&key).unwrap();
}

#[test]
fn test_serialize_key_boolean() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let mut serializer = Serializer { writer, formatter: CompactFormatter {} };
    let mut compound = Compound::Map { ser: &mut serializer, state: State::Empty };

    let key = true;
    compound.serialize_key(&key).unwrap();
}

