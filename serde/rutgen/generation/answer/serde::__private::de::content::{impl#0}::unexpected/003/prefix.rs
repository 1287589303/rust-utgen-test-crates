// Answer 0

#[test]
fn test_unexpected_newtype_struct_with_bool() {
    let content = Content::Newtype(Box::new(Content::Bool(true)));
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_newtype_struct_with_u8() {
    let content = Content::Newtype(Box::new(Content::U8(8)));
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_newtype_struct_with_string() {
    let content = Content::Newtype(Box::new(Content::String(String::from("test"))));
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_newtype_struct_with_f32() {
    let content = Content::Newtype(Box::new(Content::F32(3.14)));
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_newtype_struct_with_char() {
    let content = Content::Newtype(Box::new(Content::Char('c')));
    let _ = content.unexpected();
}

