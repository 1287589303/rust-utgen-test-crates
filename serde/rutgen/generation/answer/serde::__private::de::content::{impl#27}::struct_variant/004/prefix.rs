// Answer 0

#[test]
fn test_struct_variant_none() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct variant")
        }

        fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            unimplemented!()
        }

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            unimplemented!()
        }
    }

    let deserializer = VariantRefDeserializer::<_, TestVisitor> {
        value: None,
        err: PhantomData,
    };

    let _ = deserializer.struct_variant(&[] , TestVisitor);
}

