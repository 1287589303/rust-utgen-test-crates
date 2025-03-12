// Answer 0

#[test]
fn test_as_str_with_str() {
    let content = Content::Str("valid_utf8");
    let result = content.as_str();
}

#[test]
fn test_as_str_with_string() {
    let content = Content::String(String::from("valid_utf8"));
    let result = content.as_str();
}

