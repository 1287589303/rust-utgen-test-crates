// Answer 0

#[test]
fn test_to_string_with_non_string_keys_in_map() {
    use serde::ser::Serializer;
    use serde::Serialize;
    use std::collections::HashMap;

    struct NonStringKey;

    impl Serialize for NonStringKey {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(ser::Error::custom("failed to serialize NonStringKey"))
        }
    }

    let mut data: HashMap<NonStringKey, String> = HashMap::new();
    data.insert(NonStringKey, "value".to_string());

    let _result: Result<String> = to_string(&data);
}

#[test]
fn test_to_string_with_failing_serialize() {
    use serde::ser::Serializer;
    use serde::Serialize;

    struct FailingSerialization;

    impl Serialize for FailingSerialization {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(ser::Error::custom("serialization failed"))
        }
    }

    let value = FailingSerialization;

    let _result: Result<String> = to_string(&value);
}

