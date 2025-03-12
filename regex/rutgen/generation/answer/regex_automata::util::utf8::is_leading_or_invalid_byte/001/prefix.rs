// Answer 0

#[test]
fn test_is_leading_or_invalid_byte_0() {
    is_leading_or_invalid_byte(0x00);
}

#[test]
fn test_is_leading_or_invalid_byte_0x40() {
    is_leading_or_invalid_byte(0x40);
}

#[test]
fn test_is_leading_or_invalid_byte_0x80() {
    is_leading_or_invalid_byte(0x80);
}

#[test]
fn test_is_leading_or_invalid_byte_0xC0() {
    is_leading_or_invalid_byte(0xC0);
}

#[test]
fn test_is_leading_or_invalid_byte_0xC1() {
    is_leading_or_invalid_byte(0xC1);
}

#[test]
fn test_is_leading_or_invalid_byte_0xF5() {
    is_leading_or_invalid_byte(0xF5);
}

#[test]
fn test_is_leading_or_invalid_byte_0xF7() {
    is_leading_or_invalid_byte(0xF7);
}

#[test]
fn test_is_leading_or_invalid_byte_0xF8() {
    is_leading_or_invalid_byte(0xF8);
}

#[test]
fn test_is_leading_or_invalid_byte_0xFF() {
    is_leading_or_invalid_byte(0xFF);
}

