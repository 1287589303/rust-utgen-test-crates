// Answer 0

#[test]
fn test_deserialize_any_with_non_empty_map_and_remaining_elements() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Bool(false));

    let visitor = MockVisitor { visits: 1 };
    let result = map.deserialize_any(visitor);

    drop(result);
}

#[test]
fn test_deserialize_any_with_map_and_extra_remaining_elements() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Number(Number::from(1)));
    map.insert("key2".to_string(), Value::Number(Number::from(2)));

    let visitor = MockVisitor { visits: 1 };
    let result = map.deserialize_any(visitor);

    drop(result);
}

struct MockVisitor {
    visits: usize,
}

impl<'de> Visitor<'de> for MockVisitor {
    type Value = ();

    fn visit_map<V>(self, _map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        if self.visits > 0 {
            Ok(())
        } else {
            Err(de::Error::custom("Visitor visit_map not called"))
        }
    }
}

