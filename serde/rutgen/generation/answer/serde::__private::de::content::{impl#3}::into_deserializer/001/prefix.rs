// Answer 0

#[test]
fn test_into_deserializer_bool() {
    let content = Content::Bool(true);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_u8() {
    let content = Content::U8(255);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_u16() {
    let content = Content::U16(65535);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_u32() {
    let content = Content::U32(4294967295);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_i8() {
    let content = Content::I8(-128);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_i16() {
    let content = Content::I16(-32768);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_i32() {
    let content = Content::I32(-2147483648);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_f32() {
    let content = Content::F32(3.14159);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_f64() {
    let content = Content::F64(2.71828);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_char() {
    let content = Content::Char('c');
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_string() {
    let content = Content::String(String::from("Hello, world!"));
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_bytes() {
    let content = Content::Bytes(&[1, 2, 3]);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_none() {
    let content = Content::None;
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_some() {
    let inner_content = Content::Bool(false);
    let content = Content::Some(Box::new(inner_content));
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_unit() {
    let content = Content::Unit;
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_newtype() {
    let inner_content = Content::U32(42);
    let content = Content::Newtype(Box::new(inner_content));
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_seq() {
    let content = Content::Seq(vec![Content::U8(1), Content::U8(2), Content::U8(3)]);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

#[test]
fn test_into_deserializer_map() {
    let content = Content::Map(vec![
        (Content::String(String::from("key1")), Content::U32(1)),
        (Content::String(String::from("key2")), Content::U32(2)),
    ]);
    let deserializer: ContentRefDeserializer<_, _> = (&content).into_deserializer();
}

