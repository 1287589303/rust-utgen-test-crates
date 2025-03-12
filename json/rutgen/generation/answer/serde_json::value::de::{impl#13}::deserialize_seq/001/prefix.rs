// Answer 0

#[test]
fn test_deserialize_seq_string() {
    let value = Value::String(String::from("invalid string"));
    // Mock visitor
    struct MockVisitor;
    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array")
        }
    }
    let visitor = MockVisitor;
    let _ = value.deserialize_seq(visitor);
}

#[test]
fn test_deserialize_seq_bool() {
    let value = Value::Bool(true);
    // Mock visitor
    struct MockVisitor;
    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array")
        }
    }
    let visitor = MockVisitor;
    let _ = value.deserialize_seq(visitor);
}

#[test]
fn test_deserialize_seq_number() {
    let value = Value::Number(Number::from(42));
    // Mock visitor
    struct MockVisitor;
    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array")
        }
    }
    let visitor = MockVisitor;
    let _ = value.deserialize_seq(visitor);
}

#[test]
fn test_deserialize_seq_null() {
    let value = Value::Null;
    // Mock visitor
    struct MockVisitor;
    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array")
        }
    }
    let visitor = MockVisitor;
    let _ = value.deserialize_seq(visitor);
}

#[test]
fn test_deserialize_seq_object() {
    let value = Value::Object(Map::new());
    // Mock visitor
    struct MockVisitor;
    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array")
        }
    }
    let visitor = MockVisitor;
    let _ = value.deserialize_seq(visitor);
}

