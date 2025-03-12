// Answer 0

#[test]
fn test_into_deserializer_bool() {
    let content = Content::Bool(true);
    let deserializer = content.into_deserializer();
}

#[test]
fn test_into_deserializer_u8() {
    let content = Content::U8(255);
    let deserializer = content.into_deserializer();
}

#[test]
fn test_into_deserializer_u16() {
    let content = Content::U16(65535);
    let deserializer = content.into_deserializer();
}

#[test]
fn test_into_deserializer_i8() {
    let content = Content::I8(-128);
    let deserializer = content.into_deserializer();
}

#[test]
fn test_into_deserializer_f32() {
    let content = Content::F32(3.14);
    let deserializer = content.into_deserializer();
}

#[test]
fn test_into_deserializer_string() {
    let content = Content::String(String::from("example"));
    let deserializer = content.into_deserializer();
}

#[test]
fn test_into_deserializer_bytes() {
    let content = Content::Bytes(vec![1, 2, 3, 4, 5]);
    let deserializer = content.into_deserializer();
}

#[test]
fn test_into_deserializer_empty_seq() {
    let content = Content::Seq(vec![]);
    let deserializer = content.into_deserializer();
}

#[test]
fn test_into_deserializer_non_empty_seq() {
    let content = Content::Seq(vec![Content::U8(1), Content::U8(2)]);
    let deserializer = content.into_deserializer();
}

#[test]
fn test_into_deserializer_empty_map() {
    let content = Content::Map(vec![]);
    let deserializer = content.into_deserializer();
}

#[test]
fn test_into_deserializer_non_empty_map() {
    let content = Content::Map(vec![
        (Content::String(String::from("key1")), Content::U8(10)),
        (Content::String(String::from("key2")), Content::Bool(true)),
    ]);
    let deserializer = content.into_deserializer();
}

#[test]
fn test_into_deserializer_max_values() {
    let content = Content::U64(u64::MAX);
    let deserializer = content.into_deserializer();
}

