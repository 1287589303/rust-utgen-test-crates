// Answer 0

#[test]
fn test_new_bool() {
    let content = Content::Bool(true);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_u8() {
    let content = Content::U8(255);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_u16() {
    let content = Content::U16(65535);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_u32() {
    let content = Content::U32(4294967295);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_u64() {
    let content = Content::U64(18446744073709551615);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_i8() {
    let content = Content::I8(-128);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_i16() {
    let content = Content::I16(-32768);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_i32() {
    let content = Content::I32(-2147483648);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_i64() {
    let content = Content::I64(-9223372036854775808);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_f32() {
    let content = Content::F32(3.14);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_f64() {
    let content = Content::F64(3.141592653589793);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_char() {
    let content = Content::Char('c');
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_string() {
    let content = Content::String(String::from("hello"));
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_bytes() {
    let content = Content::Bytes(b"hello");
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_seq() {
    let content = Content::Seq(vec![Content::Bool(true), Content::U8(255)]);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_struct() {
    let content = Content::Struct("my_struct", vec![("field1", Content::I32(42))]);
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_none() {
    let content = Content::None;
    let deserializer = ContentRefDeserializer::new(&content);
}

#[test]
fn test_new_some() {
    let inner_content = Content::Bool(true);
    let content = Content::Some(Box::new(inner_content));
    let deserializer = ContentRefDeserializer::new(&content);
}

