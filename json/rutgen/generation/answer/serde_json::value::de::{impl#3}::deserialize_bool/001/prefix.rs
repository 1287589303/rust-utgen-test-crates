// Answer 0

#[test]
fn test_deserialize_bool_with_null() {
    let value = Value::Null;
    let visitor = MockVisitor;
    let _ = value.deserialize_bool(visitor);
}

#[test]
fn test_deserialize_bool_with_number() {
    let value = Value::Number(Number { n: 0 });
    let visitor = MockVisitor;
    let _ = value.deserialize_bool(visitor);
}

#[test]
fn test_deserialize_bool_with_string() {
    let value = Value::String(String::from("not a bool"));
    let visitor = MockVisitor;
    let _ = value.deserialize_bool(visitor);
}

#[test]
fn test_deserialize_bool_with_array() {
    let value = Value::Array(vec![Value::Bool(true)]);
    let visitor = MockVisitor;
    let _ = value.deserialize_bool(visitor);
}

#[test]
fn test_deserialize_bool_with_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    let visitor = MockVisitor;
    let _ = value.deserialize_bool(visitor);
}

struct MockVisitor;

impl<'de> Visitor<'de> for MockVisitor {
    type Value = ();
    
    fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(())
    }

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a boolean value")
    }
}

