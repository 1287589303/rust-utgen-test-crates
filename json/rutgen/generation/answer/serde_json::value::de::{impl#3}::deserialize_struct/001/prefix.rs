// Answer 0

#[test]
fn test_deserialize_struct_bool() {
    let value = Value::Bool(true);
    let visitor = MockVisitor;
    let _ = value.deserialize_struct("Test", &["field"], visitor);
}

#[test]
fn test_deserialize_struct_number() {
    let value = Value::Number(Number { n: 42 });
    let visitor = MockVisitor;
    let _ = value.deserialize_struct("Test", &["field"], visitor);
}

#[test]
fn test_deserialize_struct_string() {
    let value = Value::String(String::from("test"));
    let visitor = MockVisitor;
    let _ = value.deserialize_struct("Test", &["field"], visitor);
}

struct MockVisitor;

impl<'de> Visitor<'de> for MockVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("mock visitor")
    }

    fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, Error>
    where
        V: SeqAccess<'de>,
    {
        Ok(())
    }

    fn visit_map<V>(self, _map: V) -> Result<Self::Value, Error>
    where
        V: MapAccess<'de>,
    {
        Ok(())
    }
}

