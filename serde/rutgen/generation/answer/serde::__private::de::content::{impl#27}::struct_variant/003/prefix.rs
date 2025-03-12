// Answer 0

#[test]
fn test_struct_variant_with_map() {
    struct VisitorImpl;
    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let key_content = Content::String("key".to_string());
    let value_content = Content::U32(42);
    let value = Some(Content::Map(vec![(key_content, value_content)]));

    let deserializer = VariantRefDeserializer {
        value,
        err: std::marker::PhantomData::<std::convert::Infallible>,
    };

    let visitor = VisitorImpl {};
    let _ = deserializer.struct_variant(&["key"]);
}

#[test]
fn test_struct_variant_with_seq() {
    struct VisitorImpl;
    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Some(Content::Seq(vec![Content::U8(1), Content::U8(2)]));

    let deserializer = VariantRefDeserializer {
        value,
        err: std::marker::PhantomData::<std::convert::Infallible>,
    };

    let visitor = VisitorImpl {};
    let _ = deserializer.struct_variant(&["field1", "field2"]);
}

#[test]
fn test_struct_variant_with_none() {
    struct VisitorImpl;
    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }
        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }
    }

    let value: Option<Content> = None;

    let deserializer = VariantRefDeserializer {
        value,
        err: std::marker::PhantomData::<std::convert::Infallible>,
    };

    let visitor = VisitorImpl {};
    let _ = deserializer.struct_variant(&["field"]);
}

