// Answer 0

#[test]
fn test_decode_valid_base64() {
    let input = "SGVsbG8sIFdvcmxkIQ=="; // "Hello, World!" in base64
    let result = decode(input).unwrap();
    assert_eq!(result, b"Hello, World!");
}

#[test]
fn test_decode_invalid_character() {
    let input = "SGVsbG8sIFdvcmxkIQ@=="; // Invalid character '@'
    match decode(input) {
        Err(DecodeError::InvalidByte(21, b'@')) => (),
        _ => panic!("Expected InvalidByte error"),
    }
}

#[test]
fn test_decode_invalid_length() {
    let input = "SGVsbG8sIFdvcmxk"; // Padding is incorrect
    match decode(input) {
        Err(DecodeError::InvalidLength(16)) => (),
        _ => panic!("Expected InvalidLength error"),
    }
}

#[test]
fn test_decode_invalid_last_symbol() {
    let input = "SGVsbG8sIFdvcmxkIQA="; // "Hello, World!" with invalid last symbol 'A'
    match decode(input) {
        Err(DecodeError::InvalidLastSymbol(18, b'A')) => (),
        _ => panic!("Expected InvalidLastSymbol error"),
    }
}

#[test]
fn test_decode_invalid_padding() {
    let input = "SGVsbG8sIFdvcmxkIQ"; // Missing padding
    match decode(input) {
        Err(DecodeError::InvalidPadding) => (),
        _ => panic!("Expected InvalidPadding error"),
    }
}

