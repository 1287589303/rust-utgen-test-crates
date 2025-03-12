// Answer 0

#[test]
fn test_serialize_unit() {
    struct DummyWriter;
    struct DummyFormatter;

    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    let mut writer = DummyWriter;
    let formatter = DummyFormatter;
    let serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter } };

    let _result = serializer.serialize_unit();
}

#[test]
#[should_panic]
fn test_serialize_unit_with_error() {
    struct DummyWriter;
    struct DummyFormatter;

    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    let mut writer = DummyWriter;
    let formatter = DummyFormatter;
    let serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter } };

    let result = serializer.serialize_unit();
    assert!(result.is_err());
    assert_eq!(result, Err(key_must_be_a_string()));
}

