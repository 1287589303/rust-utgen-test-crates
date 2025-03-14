// Answer 0

#[test]
fn test_new_invalid_length() {
    let result = Alphabet::new("short");
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_unprintable_byte() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0");
    match result {
        Err(ParseAlphabetError::UnprintableByte(byte)) => assert_eq!(byte, 0),
        _ => panic!("Expected Err(ParseAlphabetError::UnprintableByte), got {:?}", result),
    }
}

#[test]
fn test_new_reserved_byte() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=");
    match result {
        Err(ParseAlphabetError::ReservedByte(byte)) => assert_eq!(byte, b'='),
        _ => panic!("Expected Err(ParseAlphabetError::ReservedByte), got {:?}", result),
    }
}

#[test]
fn test_new_duplicated_byte() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+AB");
    match result {
        Err(ParseAlphabetError::DuplicatedByte(byte)) => assert_eq!(byte, b'A'),
        _ => panic!("Expected Err(ParseAlphabetError::DuplicatedByte), got {:?}", result),
    }
}

