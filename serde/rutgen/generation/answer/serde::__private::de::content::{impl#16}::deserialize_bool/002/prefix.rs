// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct Visitor {
        value: bool,
    }

    impl<'de> Visitor<'de> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value, crate::de::Error> {
            Ok(value)
        }

        // Other required methods can be omitted as they won't be called in this test
    }

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };
    let visitor = Visitor { value: true };
    let _ = deserializer.deserialize_bool(visitor);
}

#[test]
fn test_deserialize_bool_false() {
    struct Visitor {
        value: bool,
    }

    impl<'de> Visitor<'de> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value, crate::de::Error> {
            Ok(value)
        }

        // Other required methods can be omitted as they won't be called in this test
    }

    let content = Content::Bool(false);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };
    let visitor = Visitor { value: false };
    let _ = deserializer.deserialize_bool(visitor);
}

