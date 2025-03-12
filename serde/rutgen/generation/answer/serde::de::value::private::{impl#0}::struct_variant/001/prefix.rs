// Answer 0

#[test]
fn test_struct_variant_with_valid_fields() {
    struct VisitorImpl;

    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(())
        }
    }

    let unit_only = UnitOnly::<std::convert::Infallible> { marker: std::marker::PhantomData };
    let fields: &'static [&'static str] = &["field1", "field2"];
    let visitor = VisitorImpl;

    let _result = unit_only.struct_variant(fields, visitor);
}

#[test]
fn test_struct_variant_with_empty_fields() {
    struct VisitorImpl;

    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(())
        }
    }

    let unit_only = UnitOnly::<std::convert::Infallible> { marker: std::marker::PhantomData };
    let fields: &'static [&'static str] = &[];
    let visitor = VisitorImpl;

    let _result = unit_only.struct_variant(fields, visitor);
}

#[test]
fn test_struct_variant_with_one_field() {
    struct VisitorImpl;

    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(())
        }
    }

    let unit_only = UnitOnly::<std::convert::Infallible> { marker: std::marker::PhantomData };
    let fields: &'static [&'static str] = &["single_field"];
    let visitor = VisitorImpl;

    let _result = unit_only.struct_variant(fields, visitor);
}

