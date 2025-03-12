// Answer 0

#[test]
fn test_unexpected_with_non_empty_bytes() {
    let content = Content::Bytes(vec![0, 255]);
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_with_empty_bytes() {
    let content = Content::Bytes(vec![]);
    let _ = content.unexpected();
}

