// Answer 0

#[test]
fn test_deserialize_ignored_any_empty_map() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }
    }

    let map = Map { map: MapImpl::<String, Value>::new() };
    let visitor = MockVisitor;
    let _result = map.deserialize_ignored_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_single_entry_map() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }
    }

    let mut entries = MapImpl::<String, Value>::new();
    entries.insert("key".to_owned(), Value::String("value".to_owned()));
    let map = Map { map: entries };
    let visitor = MockVisitor;
    let _result = map.deserialize_ignored_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_filled_map() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }
    }

    let mut entries = MapImpl::<String, Value>::new();
    entries.insert("first_key".to_owned(), Value::Bool(true));
    entries.insert("second_key".to_owned(), Value::Number(Number::from(42)));
    let map = Map { map: entries };
    let visitor = MockVisitor;
    let _result = map.deserialize_ignored_any(visitor);
}

