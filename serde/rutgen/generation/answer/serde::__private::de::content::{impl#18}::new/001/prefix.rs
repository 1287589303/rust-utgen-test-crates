// Answer 0

#[test]
fn test_enum_deserializer_with_bool_variant() {
    let variant = Content::Bool(true);
    let value = Some(Content::U8(5));
    let _: EnumDeserializer<'_, _> = EnumDeserializer::new(variant, value);
}

#[test]
fn test_enum_deserializer_with_uint_variant() {
    let variant = Content::U32(10);
    let value = Some(Content::None);
    let _: EnumDeserializer<'_, _> = EnumDeserializer::new(variant, value);
}

#[test]
fn test_enum_deserializer_with_string_variant() {
    let variant = Content::String("test".to_string());
    let value = Some(Content::Str("inner"));
    let _: EnumDeserializer<'_, _> = EnumDeserializer::new(variant, value);
}

#[test]
fn test_enum_deserializer_with_empty_option() {
    let variant = Content::Unit;
    let value = None;
    let _: EnumDeserializer<'_, _> = EnumDeserializer::new(variant, value);
}

#[test]
fn test_enum_deserializer_with_nested_content() {
    let variant = Content::Newtype(Box::new(Content::F32(3.14)));
    let value = Some(Content::Seq(vec![Content::I64(1), Content::I64(2)]));
    let _: EnumDeserializer<'_, _> = EnumDeserializer::new(variant, value);
}

