// Answer 0

#[test]
fn test_escape_utf8_non_empty_alphanumeric() {
    let input = "abcdef";
    let mut output = String::new();
    escape_utf8(input, &mut output);
}

#[test]
fn test_escape_utf8_non_empty_uppercase() {
    let input = "XYZ";
    let mut output = String::new();
    escape_utf8(input, &mut output);
}

#[test]
fn test_escape_utf8_non_empty_special_chars() {
    let input = "!@#$%^&*()";
    let mut output = String::new();
    escape_utf8(input, &mut output);
}

#[test]
fn test_escape_utf8_non_empty_mixed_chars() {
    let input = "AbCdEfG123";
    let mut output = String::new();
    escape_utf8(input, &mut output);
}

