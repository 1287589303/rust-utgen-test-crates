// Answer 0

#[test]
fn test_tuple_variant_non_empty_array() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("non-empty array")
        }

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Some(Value::Array(vec![Value::Bool(true), Value::Number(Number::from(1))]));
    let deserializer = VariantRefDeserializer { value };

    let visitor = VisitorImpl;
    let _result = deserializer.tuple_variant(2, visitor);
}

#[test]
fn test_tuple_variant_invalid_type_bool() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("tuple variant")
        }

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Some(Value::Bool(true));
    let deserializer = VariantRefDeserializer { value };

    let visitor = VisitorImpl;
    let _result = deserializer.tuple_variant(1, visitor);
}

#[test]
fn test_tuple_variant_invalid_type_string() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("tuple variant")
        }

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Some(Value::String("invalid".to_string()));
    let deserializer = VariantRefDeserializer { value };

    let visitor = VisitorImpl;
    let _result = deserializer.tuple_variant(1, visitor);
}

