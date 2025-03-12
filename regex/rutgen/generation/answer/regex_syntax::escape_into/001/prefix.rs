// Answer 0

#[test]
fn test_escape_into_empty_string() {
    let mut buf = String::new();
    escape_into("", &mut buf);
}

#[test]
fn test_escape_into_non_meta_string() {
    let mut buf = String::new();
    escape_into("abc", &mut buf);
}

#[test]
fn test_escape_into_all_meta_characters() {
    let mut buf = String::new();
    escape_into(".*+?()|[]{}^$#&-~", &mut buf);
}

#[test]
fn test_escape_into_mixed_meta_and_non_meta() {
    let mut buf = String::new();
    escape_into("a+b*c?", &mut buf);
}

#[test]
fn test_escape_into_escaped_meta_character() {
    let mut buf = String::new();
    escape_into("hello\\world", &mut buf);
}

