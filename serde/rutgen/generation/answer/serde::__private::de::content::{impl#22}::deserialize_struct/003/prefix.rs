// Answer 0

#[test]
fn test_deserialize_struct_with_empty_seq() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an empty sequence")
        }

        fn visit_seq<V>(self, _visitor: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let content = Content::Seq(Vec::new());
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };
    let _ = deserializer.deserialize_struct("TestStruct", &[], VisitorImpl);
}

#[test]
fn test_deserialize_struct_with_bool_seq() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of bools")
        }

        fn visit_seq<V>(self, visitor: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut seq_visitor = visitor.size_hint().unwrap_or(0);
            while seq_visitor.next().is_some() {
                // Consume the next item
            }
            Ok(())
        }
    }

    let content = Content::Seq(vec![Content::Bool(true), Content::Bool(false)]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };
    let _ = deserializer.deserialize_struct("TestStruct", &[], VisitorImpl);
}

#[test]
fn test_deserialize_struct_with_mixed_seq() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of mixed content types")
        }

        fn visit_seq<V>(self, visitor: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut seq_visitor = visitor.size_hint().unwrap_or(0);
            while seq_visitor.next().is_some() {
                // Consume the next item
            }
            Ok(())
        }
    }

    let content = Content::Seq(vec![
        Content::Bool(true),
        Content::I32(42),
        Content::String("Hello".to_string()),
    ]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };
    let _ = deserializer.deserialize_struct("TestStruct", &[], VisitorImpl);
}

