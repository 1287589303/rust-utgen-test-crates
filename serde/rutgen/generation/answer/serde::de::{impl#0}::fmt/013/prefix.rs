// Answer 0

#[test]
fn test_unexpected_str_non_empty() {
    let unexpected = Unexpected::Str("non-empty string");
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

#[test]
fn test_unexpected_str_empty() {
    let unexpected = Unexpected::Str("");
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

#[test]
fn test_unexpected_str_special_characters() {
    let unexpected = Unexpected::Str("string with special characters! @#$%^&*()");
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

#[test]
fn test_unexpected_str_whitespace() {
    let unexpected = Unexpected::Str("   ");
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

#[test]
fn test_unexpected_str_unicode() {
    let unexpected = Unexpected::Str("string with unicode: 你好");
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

