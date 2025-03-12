// Answer 0

#[test]
fn test_deserialize_ignored_any_with_null() {
    let value = Value::Null;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }
    let visitor = TestVisitor;
    let _ = value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_with_bool() {
    let value = Value::Bool(true);
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }
    let visitor = TestVisitor;
    let _ = value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_with_number() {
    let value = Value::Number(Number::from_f64(42.0).unwrap());
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }
    let visitor = TestVisitor;
    let _ = value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_with_string() {
    let value = Value::String(String::from("test"));
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }
    let visitor = TestVisitor;
    let _ = value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_with_array() {
    let value = Value::Array(vec![Value::Bool(false), Value::String(String::from("item"))]);
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }
    let visitor = TestVisitor;
    let _ = value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_with_object() {
    let mut obj = Map::new();
    obj.insert(String::from("key"), Value::Number(Number::from_f64(5.0).unwrap()));
    let value = Value::Object(obj);
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }
    let visitor = TestVisitor;
    let _ = value.deserialize_any(visitor);
}

