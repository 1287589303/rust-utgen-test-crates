// Answer 0

#[test]
fn test_deserialize_ignored_any_with_null_value() {
    let value: &Value = &Value::Null;
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let _ = value.deserialize_ignored_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_with_boolean_value() {
    let value: &Value = &Value::Bool(true);
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let _ = value.deserialize_ignored_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_with_number_value() {
    let value: &Value = &Value::Number(Number::from(42));
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let _ = value.deserialize_ignored_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_with_string_value() {
    let value: &Value = &Value::String(String::from("test"));
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let _ = value.deserialize_ignored_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_with_array_value() {
    let value: &Value = &Value::Array(vec![Value::Bool(true), Value::Number(Number::from(1))]);
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let _ = value.deserialize_ignored_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_with_object_value() {
    let value: &Value = &Value::Object(Map::new());
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let _ = value.deserialize_ignored_any(visitor);
}

