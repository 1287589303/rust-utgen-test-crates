// Answer 0

#[test]
fn test_struct_variant_with_empty_map() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an empty map")
        }
        fn visit_map<M>(self, _map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Some(Content::Map(vec![]));
    let deserializer = VariantDeserializer { value, err: PhantomData };
    let _ = deserializer.struct_variant(&[], TestVisitor);
}

#[test]
fn test_struct_variant_with_non_empty_map() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a non-empty map")
        }
        fn visit_map<M>(self, _map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Some(Content::Map(vec![
        (Content::String("key1".to_string()), Content::U32(1)),
        (Content::String("key2".to_string()), Content::U32(2)),
    ]));
    let deserializer = VariantDeserializer { value, err: PhantomData };
    let _ = deserializer.struct_variant(&[], TestVisitor);
}

#[test]
fn test_struct_variant_with_empty_sequence() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an empty sequence")
        }
        fn visit_seq<S>(self, _seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Some(Content::Seq(vec![]));
    let deserializer = VariantDeserializer { value, err: PhantomData };
    let _ = deserializer.struct_variant(&[], TestVisitor);
}

#[test]
fn test_struct_variant_with_non_empty_sequence() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a non-empty sequence")
        }
        fn visit_seq<S>(self, _seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Some(Content::Seq(vec![
        Content::U32(1),
        Content::U32(2),
        Content::U32(3),
    ]));
    let deserializer = VariantDeserializer { value, err: PhantomData };
    let _ = deserializer.struct_variant(&[], TestVisitor);
}

#[test]
fn test_struct_variant_with_unexpected_content() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unexpected content")
        }
        fn visit_seq<S>(self, _seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Ok(())
        }
        fn visit_map<M>(self, _map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Some(Content::I32(42));
    let deserializer = VariantDeserializer { value, err: PhantomData };
    let _ = deserializer.struct_variant(&[], TestVisitor);
}

#[test]
fn test_struct_variant_with_none() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("none")
        }
        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }
    }

    let value: Option<Content> = None;
    let deserializer = VariantDeserializer { value, err: PhantomData };
    let _ = deserializer.struct_variant(&[], TestVisitor);
}

