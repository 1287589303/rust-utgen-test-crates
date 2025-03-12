// Answer 0

#[test]
fn test_unexpected_with_non_empty_string() {
    let content = Content::String(String::from("test string"));
    let result = content.unexpected();
}

#[test]
fn test_unexpected_with_empty_string() {
    let content = Content::String(String::from(""));
    let result = content.unexpected();
}

