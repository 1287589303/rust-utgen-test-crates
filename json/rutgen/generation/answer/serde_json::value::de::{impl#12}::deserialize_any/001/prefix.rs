// Answer 0

#[test]
fn test_deserialize_any_with_error_visitor() {
    struct ErrorVisitor;

    impl<'de> Visitor<'de> for ErrorVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with more elements")
        }

        fn visit_map<M>(self, _: M) -> Result<Self::Value, serde::de::Error>
        where
            M: MapAccess<'de>,
        {
            Err(serde::de::Error::custom("visit_map error"))
        }
    }

    let mut map = Map::new();
    map.insert("key1".to_owned(), Value::Bool(true));
    
    let deserializer: &Map<String, Value> = &map;
    let visitor = ErrorVisitor;

    let result = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_with_non_empty_map_error() {
    struct ErrorVisitor;

    impl<'de> Visitor<'de> for ErrorVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with more elements")
        }

        fn visit_map<M>(self, _: M) -> Result<Self::Value, serde::de::Error>
        where
            M: MapAccess<'de>,
        {
            Err(serde::de::Error::custom("visit_map error"))
        }
    }

    let mut map = Map::with_capacity(2);
    map.insert("key1".to_owned(), Value::Number(Number::from(1)));
    map.insert("key2".to_owned(), Value::String("value".to_owned()));

    let deserializer: &Map<String, Value> = &map;
    let visitor = ErrorVisitor;

    let result = deserializer.deserialize_any(visitor);
}

