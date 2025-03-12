// Answer 0

#[test]
fn test_tuple_variant_with_map() {
    struct VisitorImpl;

    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            unimplemented!()
        }
    }

    let content = Content::Map(vec![(Content::String("key".to_string()), Content::U32(1))]);
    let deserializer = VariantRefDeserializer {
        value: Some(&content),
        err: PhantomData,
    };

    let visitor = VisitorImpl;
    let _result = deserializer.tuple_variant(2, visitor);
}

#[test]
fn test_tuple_variant_with_non_seq_content() {
    struct VisitorImpl;

    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            unimplemented!()
        }
    }

    let content = Content::String("not a sequence".to_string());
    let deserializer = VariantRefDeserializer {
        value: Some(&content),
        err: PhantomData,
    };

    let visitor = VisitorImpl;
    let _result = deserializer.tuple_variant(1, visitor);
}

