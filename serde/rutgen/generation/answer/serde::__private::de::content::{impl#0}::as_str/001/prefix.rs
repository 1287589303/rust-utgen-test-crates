// Answer 0

#[test]
fn test_as_str_with_bool() {
    let content = Content::Bool(true);
    content.as_str();
}

#[test]
fn test_as_str_with_u8() {
    let content = Content::U8(0);
    content.as_str();
}

#[test]
fn test_as_str_with_i32() {
    let content = Content::I32(0);
    content.as_str();
}

#[test]
fn test_as_str_with_none() {
    let content = Content::None;
    content.as_str();
}

#[test]
fn test_as_str_with_unit() {
    let content = Content::Unit;
    content.as_str();
}

