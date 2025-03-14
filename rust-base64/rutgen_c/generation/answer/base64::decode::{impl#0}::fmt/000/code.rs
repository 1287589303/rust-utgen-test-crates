// Answer 0

#[test]
fn test_invalid_byte_display() {
    let error = DecodeError::InvalidByte(5, b'A');
    let result = format!("{}", error);
    assert_eq!(result, "Invalid symbol 65, offset 5.");
}

#[test]
fn test_invalid_length_display() {
    let error = DecodeError::InvalidLength(3);
    let result = format!("{}", error);
    assert_eq!(result, "Invalid input length: 3");
}

#[test]
fn test_invalid_last_symbol_display() {
    let error = DecodeError::InvalidLastSymbol(4, b'Z');
    let result = format!("{}", error);
    assert_eq!(result, "Invalid last symbol 90, offset 4.");
}

#[test]
fn test_invalid_padding_display() {
    let error = DecodeError::InvalidPadding;
    let result = format!("{}", error);
    assert_eq!(result, "Invalid padding");
}

