// Answer 0

#[test]
fn test_invalid_length_display() {
    let error = DecodeError::InvalidLength(5);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid input length: 5");
}

#[test]
fn test_invalid_byte_display() {
    let error = DecodeError::InvalidByte(2, b'&');
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid symbol &, offset 2.");
}

#[test]
fn test_invalid_last_symbol_display() {
    let error = DecodeError::InvalidLastSymbol(3, b'X');
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid last symbol X, offset 3.");
}

#[test]
fn test_invalid_padding_display() {
    let error = DecodeError::InvalidPadding;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid padding");
}

