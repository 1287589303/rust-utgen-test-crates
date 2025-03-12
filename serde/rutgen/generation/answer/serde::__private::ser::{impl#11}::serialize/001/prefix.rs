// Answer 0

#[test]
fn test_serialize_adjacent_variant_valid_input() {
    let variant = AdjacentlyTaggedEnumVariant {
        enum_name: "TestEnum",
        variant_index: 0,
        variant_name: "VariantA",
    };
    let mock_serializer = MockSerializer::new();
    let _ = variant.serialize(mock_serializer);
}

#[test]
fn test_serialize_adjacent_variant_max_index() {
    let variant = AdjacentlyTaggedEnumVariant {
        enum_name: "TestEnum",
        variant_index: u32::MAX,
        variant_name: "VariantMax",
    };
    let mock_serializer = MockSerializer::new();
    let _ = variant.serialize(mock_serializer);
}

#[test]
fn test_serialize_adjacent_variant_non_empty_strings() {
    let variant = AdjacentlyTaggedEnumVariant {
        enum_name: "AnotherEnum",
        variant_index: 1,
        variant_name: "VariantB",
    };
    let mock_serializer = MockSerializer::new();
    let _ = variant.serialize(mock_serializer);
}

