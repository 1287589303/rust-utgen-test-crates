// Answer 0

#[test]
fn test_deserialize_char_with_string() {
    struct TestVisitor {
        value: Option<char>,
    }

    impl Visitor<'_> for TestVisitor {
        type Value = Option<char>;

        fn visit_char(self, value: char) -> Result<Self::Value, std::convert::Infallible> {
            Ok(Some(value))
        }

        fn visit_str(self, value: &str) -> Result<Self::Value, std::convert::Infallible> {
            Ok(value.chars().next())
        }

        fn visit_borrowed_str(self, value: &'_ str) -> Result<Self::Value, std::convert::Infallible> {
            Ok(value.chars().next())
        }

        fn visit_bytes(self, _value: &[u8]) -> Result<Self::Value, std::convert::Infallible> {
            Err(std::convert::Infallible)
        }

        fn visit_borrowed_bytes(self, _value: &'_ [u8]) -> Result<Self::Value, std::convert::Infallible> {
            Err(std::convert::Infallible)
        }

        fn visit_unit(self) -> Result<Self::Value, std::convert::Infallible> {
            Ok(None)
        }

        // other required methods can be no-op for this test
    }

    let content = Content::String("A".to_string());
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let visitor = TestVisitor { value: None };
    let _ = deserializer.deserialize_char(visitor);
}

#[test]
fn test_deserialize_char_with_str() {
    struct TestVisitor {
        value: Option<char>,
    }

    impl Visitor<'_> for TestVisitor {
        type Value = Option<char>;

        fn visit_char(self, value: char) -> Result<Self::Value, std::convert::Infallible> {
            Ok(Some(value))
        }

        fn visit_str(self, value: &str) -> Result<Self::Value, std::convert::Infallible> {
            Ok(value.chars().next())
        }

        fn visit_borrowed_str(self, value: &'_ str) -> Result<Self::Value, std::convert::Infallible> {
            Ok(value.chars().next())
        }

        fn visit_bytes(self, _value: &[u8]) -> Result<Self::Value, std::convert::Infallible> {
            Err(std::convert::Infallible)
        }

        fn visit_borrowed_bytes(self, _value: &'_ [u8]) -> Result<Self::Value, std::convert::Infallible> {
            Err(std::convert::Infallible)
        }

        fn visit_unit(self) -> Result<Self::Value, std::convert::Infallible> {
            Ok(None)
        }

        // other required methods can be no-op for this test
    }

    let content = Content::Str("B");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let visitor = TestVisitor { value: None };
    let _ = deserializer.deserialize_char(visitor);
}

#[test]
#[should_panic]
fn test_deserialize_char_with_invalid_content() {
    struct TestVisitor {
        value: Option<char>,
    }

    impl Visitor<'_> for TestVisitor {
        type Value = Option<char>;

        fn visit_char(self, value: char) -> Result<Self::Value, std::convert::Infallible> {
            Ok(Some(value))
        }

        fn visit_str(self, value: &str) -> Result<Self::Value, std::convert::Infallible> {
            Ok(value.chars().next())
        }

        fn visit_borrowed_str(self, value: &'_ str) -> Result<Self::Value, std::convert::Infallible> {
            Ok(value.chars().next())
        }

        fn visit_bytes(self, _value: &[u8]) -> Result<Self::Value, std::convert::Infallible> {
            Err(std::convert::Infallible)
        }

        fn visit_borrowed_bytes(self, _value: &'_ [u8]) -> Result<Self::Value, std::convert::Infallible> {
            Err(std::convert::Infallible)
        }

        fn visit_unit(self) -> Result<Self::Value, std::convert::Infallible> {
            Ok(None)
        }

        // other required methods can be no-op for this test
    }

    let content = Content::U64(12345);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let visitor = TestVisitor { value: None };
    let _ = deserializer.deserialize_char(visitor);
}

