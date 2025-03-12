// Answer 0

#[test]
fn test_serialize_entry_key_error() {
    struct TestError;
    impl ser::Error for TestError {}

    struct FailingSerialize;

    impl Serialize for FailingSerialize {
        fn serialize<S>(&self, _: S) -> Result<(), TestError>
        where
            S: Serializer,
        {
            Err(TestError)
        }
    }

    struct ValidValue;

    impl Serialize for ValidValue {
        fn serialize<S>(&self, _: S) -> Result<(), TestError>
        where
            S: Serializer,
        {
            Ok(())
        }
    }

    let mut serializer = SerializeMap::<TestError> {
        entries: Vec::new(),
        key: None,
        error: PhantomData,
    };

    let key = FailingSerialize;
    let value = ValidValue;

    let result = serializer.serialize_entry(&key, &value);
}

#[test]
fn test_serialize_entry_key_error_with_valid_value() {
    struct TestError;
    impl ser::Error for TestError {}

    struct AnotherFailingSerialize;

    impl Serialize for AnotherFailingSerialize {
        fn serialize<S>(&self, _: S) -> Result<(), TestError>
        where
            S: Serializer,
        {
            Err(TestError)
        }
    }

    struct AnotherValidValue;

    impl Serialize for AnotherValidValue {
        fn serialize<S>(&self, _: S) -> Result<(), TestError>
        where
            S: Serializer,
        {
            Ok(())
        }
    }

    let mut serializer = SerializeMap::<TestError> {
        entries: Vec::new(),
        key: None,
        error: PhantomData,
    };

    let key = AnotherFailingSerialize;
    let value = AnotherValidValue;

    let result = serializer.serialize_entry(&key, &value);
}

