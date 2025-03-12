// Answer 0

#[test]
fn test_escape_utf8_with_null_and_digit_following() {
    let mut output = String::new();
    let input = "hello\0world7";
    escape_utf8(input, &mut output);
}

#[test]
fn test_escape_utf8_with_null_and_digit_leading() {
    let mut output = String::new();
    let input = "\0world3hello";
    escape_utf8(input, &mut output);
}

#[test]
fn test_escape_utf8_with_single_quote() {
    let mut output = String::new();
    let input = "hello'world";
    escape_utf8(input, &mut output);
}

#[test]
fn test_escape_utf8_with_null_and_single_quote() {
    let mut output = String::new();
    let input = "\0'world";
    escape_utf8(input, &mut output);
}

#[test]
fn test_escape_utf8_with_mixed_characters() {
    let mut output = String::new();
    let input = "tes't\0string4";
    escape_utf8(input, &mut output);
}

