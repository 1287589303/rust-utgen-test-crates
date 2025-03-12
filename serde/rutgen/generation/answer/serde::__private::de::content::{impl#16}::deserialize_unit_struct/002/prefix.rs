// Answer 0

#[test]
fn test_deserialize_unit_struct_empty_map() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }
    }

    let deserializer = ContentDeserializer {
        content: Content::Map(Vec::new()),
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_unit_struct("test", VisitorImpl);
}

#[test]
fn test_deserialize_unit_struct_empty_seq() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }
    }

    let deserializer = ContentDeserializer {
        content: Content::Seq(Vec::new()),
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_unit_struct("test", VisitorImpl);
}

