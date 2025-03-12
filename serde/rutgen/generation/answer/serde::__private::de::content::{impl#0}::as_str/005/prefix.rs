// Answer 0

#[test]
fn test_as_str_string() {
    let content = Content::String(String::from("test string"));
    let result = content.as_str();
}

#[test]
fn test_as_str_bytes() {
    let content = Content::Bytes(b"test bytes".to_vec());
    let result = content.as_str();
}

#[test]
fn test_as_str_byte_buf() {
    let content = Content::ByteBuf(vec![116, 101, 115, 116, 32, 98, 121, 116, 101]);
    let result = content.as_str();
}

