// Answer 0

#[test]
fn test_deserialize_any_with_empty_array() {
    let visitor = MyVisitor {};
    let value = Value::Array(Vec::new());
    let _ = value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_with_single_element_array() {
    let visitor = MyVisitor {};
    let value = Value::Array(vec![Value::Bool(true)]);
    let _ = value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_with_multiple_elements_array() {
    let visitor = MyVisitor {};
    let value = Value::Array(vec![
        Value::Null,
        Value::Bool(false),
        Value::Number(Number { n: 42 }),
        Value::String(String::from("test")),
    ]);
    let _ = value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_with_nested_array() {
    let visitor = MyVisitor {};
    let value = Value::Array(vec![
        Value::Array(vec![Value::Bool(true)]),
        Value::Array(vec![Value::Number(Number { n: 3.14 })]),
    ]);
    let _ = value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_with_large_array() {
    let visitor = MyVisitor {};
    let value = Value::Array((0..1000).map(|_| Value::Number(Number { n: 1 })).collect());
    let _ = value.deserialize_any(visitor);
}

struct MyVisitor;

impl<'de> Visitor<'de> for MyVisitor {
    type Value = ();

    fn visit_unit(self) -> Result<Self::Value, Error> {
        Ok(())
    }

    fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
        Ok(())
    }

    fn visit_string(self, _: String) -> Result<Self::Value, Error> {
        Ok(())
    }

    fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, Error>
    where
        V: SeqAccess<'de>,
    {
        Ok(())
    }

    fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, Error>
    where
        V: MapAccess<'de>,
    {
        Ok(())
    }
}

