// Answer 0

#[test]
fn test_digit_none_below_range() {
    let byte: u8 = 0; // value is below the range
    let result = byte.digit();
}

#[test]
fn test_digit_none_above_range() {
    let byte: u8 = 123; // value is above the range
    let result = byte.digit();
}

#[test]
fn test_digit_none_non_ascii() {
    let byte: u8 = 255; // value is outside the ASCII range
    let result = byte.digit();
}

