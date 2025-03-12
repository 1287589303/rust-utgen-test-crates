// Answer 0

#[test]
fn test_deserialize_byte_buf_with_valid_bytes() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }
    }

    let value = Value::Array(vec![Value::Number(Number::from(1)), Value::Number(Number::from(2))]);
    let mut visitor = TestVisitor;
    let _ = value.deserialize_byte_buf(&mut visitor);
}

#[test]
fn test_deserialize_byte_buf_with_empty_byte_array() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }
    }

    let value = Value::Array(vec![]);
    let mut visitor = TestVisitor;
    let _ = value.deserialize_byte_buf(&mut visitor);
}

#[test]
#[should_panic]
fn test_deserialize_byte_buf_with_malformed_bytes() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            panic!("This visitor is not meant to handle valid bytes");
        }
    }

    let value = Value::String(String::from("not a bytes representation"));
    let mut visitor = TestVisitor;
    let _ = value.deserialize_byte_buf(&mut visitor);
}

