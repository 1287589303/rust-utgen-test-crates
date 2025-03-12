// Answer 0

#[test]
fn test_serialize_entry_key_ok_value_err() {
    struct TestError;
    impl std::fmt::Debug for TestError {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    impl serde::ser::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct KeyOk;
    impl serde::ser::Serialize for KeyOk {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            Ok(Content::String("key".to_string()))
        }
    }

    struct ValueErr;
    impl serde::ser::Serialize for ValueErr {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            Err(TestError)
        }
    }

    let mut map = SerializeMap::<TestError> {
        entries: vec![],
        key: None,
        error: std::marker::PhantomData,
    };

    let key = KeyOk;
    let value = ValueErr;

    let result = map.serialize_entry(&key, &value);
}

