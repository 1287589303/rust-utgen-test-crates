// Answer 0

#[test]
fn test_serialize_value_with_err() {
    struct TestError;
    impl ser::Error for TestError {}

    struct FailingSerialize;

    impl Serialize for FailingSerialize {
        fn serialize<S>(&self, _serializer: S) -> Result<Self::Ok, Self::Error>
        where
            S: Serializer,
        {
            Err(TestError)
        }
    }

    let mut map = SerializeMap::<TestError> {
        entries: Vec::new(),
        key: Some(Content::String("key".to_string())),
        error: PhantomData,
    };

    let value = FailingSerialize;

    let result = map.serialize_value(&value);
}

#[test]
fn test_serialize_value_with_different_err() {
    struct AnotherTestError;
    impl ser::Error for AnotherTestError {}

    struct AnotherFailingSerialize;

    impl Serialize for AnotherFailingSerialize {
        fn serialize<S>(&self, _serializer: S) -> Result<Self::Ok, Self::Error>
        where
            S: Serializer,
        {
            Err(AnotherTestError)
        }
    }

    let mut map = SerializeMap::<AnotherTestError> {
        entries: Vec::new(),
        key: Some(Content::U32(42)),
        error: PhantomData,
    };

    let value = AnotherFailingSerialize;

    let result = map.serialize_value(&value);
}

