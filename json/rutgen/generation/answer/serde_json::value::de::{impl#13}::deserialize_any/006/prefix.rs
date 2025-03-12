// Answer 0

#[test]
fn test_deserialize_any_null() {
    let value = serde_json::Value::Null;
    let visitor = TestVisitor::new();
    let result = value.deserialize_any(visitor);
}

struct TestVisitor;

impl TestVisitor {
    fn new() -> Self {
        TestVisitor
    }
}

impl<'de> serde::de::Visitor<'de> for TestVisitor {
    type Value = ();

    fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
        Ok(())
    }

    fn visit_bool(self, _: bool) -> Result<Self::Value, serde::de::Error> {
        unreachable!()
    }

    fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, serde::de::Error> {
        unreachable!()
    }

    fn visit_seq<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        unreachable!()
    }

    fn visit_map<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        unreachable!()
    }
}

