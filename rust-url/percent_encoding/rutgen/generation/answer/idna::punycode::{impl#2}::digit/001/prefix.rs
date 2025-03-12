// Answer 0

#[test]
fn test_digit_with_numeric_byte() {
    let byte: u8 = b'0';
    let result = byte.digit();
}

#[test]
fn test_digit_with_another_numeric_byte() {
    let byte: u8 = b'5';
    let result = byte.digit();
}

#[test]
fn test_digit_with_uppercase_letter() {
    let byte: u8 = b'A';
    let result = byte.digit();
}

#[test]
fn test_digit_with_another_uppercase_letter() {
    let byte: u8 = b'Z';
    let result = byte.digit();
}

#[test]
fn test_digit_with_lowercase_letter() {
    let byte: u8 = b'a';
    let result = byte.digit();
}

#[test]
fn test_digit_with_another_lowercase_letter() {
    let byte: u8 = b'z';
    let result = byte.digit();
}

