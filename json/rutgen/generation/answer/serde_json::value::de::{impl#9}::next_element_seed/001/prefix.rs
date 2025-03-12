// Answer 0

#[test]
fn test_next_element_seed_with_null() {
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = Value;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            // Assume a simple pass-through
            deserializer.deserialize_any(Visitor)
        }
    }

    let values = vec![Value::Null];
    let mut deserializer = SeqDeserializer {
        iter: values.into_iter(),
    };

    let _ = deserializer.next_element_seed(TestSeed);
}

#[test]
fn test_next_element_seed_with_bool() {
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = Value;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(Visitor)
        }
    }

    let values = vec![Value::Bool(true)];
    let mut deserializer = SeqDeserializer {
        iter: values.into_iter(),
    };

    let _ = deserializer.next_element_seed(TestSeed);
}

#[test]
fn test_next_element_seed_with_number() {
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = Value;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(Visitor)
        }
    }

    let values = vec![Value::Number(Number::from(12.5))];
    let mut deserializer = SeqDeserializer {
        iter: values.into_iter(),
    };

    let _ = deserializer.next_element_seed(TestSeed);
}

#[test]
fn test_next_element_seed_with_string() {
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = Value;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(Visitor)
        }
    }

    let values = vec![Value::String(String::from("test"))];
    let mut deserializer = SeqDeserializer {
        iter: values.into_iter(),
    };

    let _ = deserializer.next_element_seed(TestSeed);
}

#[test]
fn test_next_element_seed_with_array() {
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = Value;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(Visitor)
        }
    }

    let values = vec![Value::Array(vec![Value::String(String::from("item"))])];
    let mut deserializer = SeqDeserializer {
        iter: values.into_iter(),
    };

    let _ = deserializer.next_element_seed(TestSeed);
}

#[test]
fn test_next_element_seed_with_object() {
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = Value;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(Visitor)
        }
    }

    let values = vec![Value::Object(Map::new())];
    let mut deserializer = SeqDeserializer {
        iter: values.into_iter(),
    };

    let _ = deserializer.next_element_seed(TestSeed);
}

