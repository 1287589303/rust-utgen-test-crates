// Answer 0

#[test]
fn test_escape_into_empty_string() {
    let mut buf = String::new();
    escape_into("", &mut buf);
}

#[test]
fn test_escape_into_no_meta_characters_short() {
    let mut buf = String::new();
    escape_into("abc", &mut buf);
}

#[test]
fn test_escape_into_no_meta_characters_medium() {
    let mut buf = String::new();
    escape_into("abcdefg", &mut buf);
}

#[test]
fn test_escape_into_no_meta_characters_long() {
    let mut buf = String::new();
    escape_into("abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij", &mut buf);
}

#[test]
fn test_escape_into_no_meta_characters_boundary() {
    let mut buf = String::new();
    escape_into("a", &mut buf);
} 

#[test]
fn test_escape_into_no_meta_characters_max_length() {
    let mut buf = String::new();
    let text = "x".repeat(100);
    escape_into(&text, &mut buf);
}

