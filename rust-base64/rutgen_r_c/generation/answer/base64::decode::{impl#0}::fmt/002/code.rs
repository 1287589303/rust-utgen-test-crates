// Answer 0

#[test]
fn test_invalid_last_symbol_display() {
    let error = DecodeError::InvalidLastSymbol(5, b'@');
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid last symbol @, offset 5.");
}

#[test]
fn test_invalid_last_symbol_invalid_byte() {
    let error = DecodeError::InvalidLastSymbol(10, b'#');
    let mut output = String::new();
    let result = error.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid last symbol #, offset 10.");
}

