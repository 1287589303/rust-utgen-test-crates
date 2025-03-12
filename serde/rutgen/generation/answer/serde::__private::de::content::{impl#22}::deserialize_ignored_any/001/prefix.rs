// Answer 0

#[test]
fn test_deserialize_ignored_any_with_valid_visitor() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, crate::de::Error> {
            Ok(())
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, crate::de::Error> {
            Err(crate::de::Error::custom("not a bool"))
        }
        
        // Other visitor methods would go here but are not necessary for this test
    }

    let content = crate::Content::Unit;
    let deserializer = crate::ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_ignored_any(TestVisitor);
}

#[test]
#[should_panic(expected = "not a bool")]
fn test_deserialize_ignored_any_with_invalid_visitor() {
    struct InvalidVisitor;

    impl<'de> Visitor<'de> for InvalidVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, crate::de::Error> {
            unreachable!()
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, crate::de::Error> {
            Err(crate::de::Error::custom("not a bool"))
        }

        // Other visitor methods would go here but are not necessary for this test
    }

    let content = crate::Content::Unit;
    let deserializer = crate::ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_ignored_any(InvalidVisitor);
}

