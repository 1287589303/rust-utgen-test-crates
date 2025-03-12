// Answer 0

#[test]
fn test_deserialize_newtype_struct_null() {
    let value = Value::Null;
    let name = "null_type";

    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_newtype_struct<E>(self, _: Value) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let _ = value.deserialize_newtype_struct(name, TestVisitor);
}

#[test]
fn test_deserialize_newtype_struct_bool() {
    let value = Value::Bool(true);
    let name = "bool_type";

    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_newtype_struct<E>(self, _: Value) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let _ = value.deserialize_newtype_struct(name, TestVisitor);
}

#[test]
fn test_deserialize_newtype_struct_number() {
    let value = Value::Number(Number::from_f64(42.0).unwrap());
    let name = "number_type";

    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_newtype_struct<E>(self, _: Value) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let _ = value.deserialize_newtype_struct(name, TestVisitor);
}

#[test]
fn test_deserialize_newtype_struct_string() {
    let value = Value::String("test".to_string());
    let name = "string_type";

    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_newtype_struct<E>(self, _: Value) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let _ = value.deserialize_newtype_struct(name, TestVisitor);
}

#[test]
fn test_deserialize_newtype_struct_array() {
    let value = Value::Array(vec![Value::Bool(false), Value::Number(Number::from_f64(3.14).unwrap())]);
    let name = "array_type";

    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_newtype_struct<E>(self, _: Value) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let _ = value.deserialize_newtype_struct(name, TestVisitor);
}

#[test]
fn test_deserialize_newtype_struct_object() {
    let value = Value::Object(Map::new());
    let name = "object_type";

    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_newtype_struct<E>(self, _: Value) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let _ = value.deserialize_newtype_struct(name, TestVisitor);
}

