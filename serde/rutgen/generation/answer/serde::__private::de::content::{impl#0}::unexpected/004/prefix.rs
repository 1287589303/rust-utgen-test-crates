// Answer 0

#[test]
fn test_unexpected_unit() {
    let content = Content::Unit;
    let result = content.unexpected();
}

#[test]
fn test_unexpected_none() {
    let content = Content::None;
    let result = content.unexpected();
}

#[test]
fn test_unexpected_some() {
    let content = Content::Some(Box::new(Content::Unit));
    let result = content.unexpected();
}

