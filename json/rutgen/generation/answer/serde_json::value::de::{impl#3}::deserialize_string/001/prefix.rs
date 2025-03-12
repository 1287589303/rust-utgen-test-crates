// Answer 0

#[test]
fn test_deserialize_string_null() {
    let value = Value::Null;
    let visitor = MockVisitor;
    let _ = value.deserialize_string(visitor);
}

#[test]
fn test_deserialize_string_bool() {
    let value = Value::Bool(true);
    let visitor = MockVisitor;
    let _ = value.deserialize_string(visitor);
}

#[test]
fn test_deserialize_string_number() {
    let value = Value::Number(Number { n: 0 });
    let visitor = MockVisitor;
    let _ = value.deserialize_string(visitor);
}

#[test]
fn test_deserialize_string_array() {
    let value = Value::Array(vec![Value::Bool(false)]);
    let visitor = MockVisitor;
    let _ = value.deserialize_string(visitor);
}

#[test]
fn test_deserialize_string_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    let visitor = MockVisitor;
    let _ = value.deserialize_string(visitor);
}

struct MockVisitor;

impl<'de> Visitor<'de> for MockVisitor {
    type Value = ();
    fn visit_string<E>(self, _value: String) -> Result<Self::Value, E> where E: de::Error { 
        unimplemented!(); 
    }
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }
}

