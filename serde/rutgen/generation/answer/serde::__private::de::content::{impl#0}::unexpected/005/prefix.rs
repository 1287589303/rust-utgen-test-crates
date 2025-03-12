// Answer 0

#[test]
fn test_unexpected_with_none() {
    let content = Content::None;
    content.unexpected();
}

#[test]
fn test_unexpected_with_some_bool() {
    let content = Content::Some(Box::new(Content::Bool(true)));
    content.unexpected();
}

#[test]
fn test_unexpected_with_some_u32() {
    let content = Content::Some(Box::new(Content::U32(0)));
    content.unexpected();
}

#[test]
fn test_unexpected_with_some_string() {
    let content = Content::Some(Box::new(Content::String("test".to_string())));
    content.unexpected();
}

