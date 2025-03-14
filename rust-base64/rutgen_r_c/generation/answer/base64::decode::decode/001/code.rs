// Answer 0

#[test]
fn test_decode_valid_input() {
    let input = "U29tZSB2YWxpZCBpbmJ1dHM="; // "Some valid inputs" in base64
    let result = decode(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"Some valid inputs".to_vec());
}

#[test]
fn test_decode_invalid_characters() {
    let input = "U29tZSB2YWxpZCBpbmJ1dHM@"; // Invalid character '@'
    let result = decode(input);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(offset, byte)) = result {
        assert_eq!(offset, 22);
        assert_eq!(byte, b'@');
    } else {
        panic!("Expected DecodeError::InvalidByte");
    }
}

#[test]
fn test_decode_invalid_length() {
    let input = "U29t"; // "Some" base64, but needs padding 
    let result = decode(input);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidLength(length)) = result {
        assert_eq!(length, 3);
    } else {
        panic!("Expected DecodeError::InvalidLength");
    }
}

#[test]
fn test_decode_invalid_last_symbol() {
    let input = "U29tZSB2YWxpZCBpbmJ1dHN"; // The last symbol is cut off
    let result = decode(input);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidLastSymbol(offset, byte)) = result {
        assert_eq!(offset, 22);
        assert_eq!(byte, b'N');
    } else {
        panic!("Expected DecodeError::InvalidLastSymbol");
    }
}

#[test]
fn test_decode_invalid_padding() {
    let input = "U29tZSB2YWxpZCBpbmJ1dHM==="; // Too many padding characters
    let result = decode(input);
    assert!(result.is_err());
    assert_eq!(result, Err(DecodeError::InvalidPadding));
}

