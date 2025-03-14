// Answer 0

#[test]
fn test_fmt_invalid_byte() {
    let error = DecodeError::InvalidByte(5, b'x');
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid symbol x, offset 5.");
}

#[test]
fn test_fmt_invalid_length() {
    let error = DecodeError::InvalidLength(3);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid input length: 3");
}

#[test]
fn test_fmt_invalid_last_symbol() {
    let error = DecodeError::InvalidLastSymbol(7, b'y');
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid last symbol y, offset 7.");
}

#[test]
fn test_fmt_invalid_padding() {
    let error = DecodeError::InvalidPadding;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid padding");
}

