// Answer 0

#[test]
fn test_new_valid_alphabet() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert!(result.is_ok());
}

#[test]
fn test_new_invalid_length() {
    let result = Alphabet::new("short");
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_unprintable_byte() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/[\n]");
    if let Err(ParseAlphabetError::UnprintableByte(byte)) = result {
        assert_eq!(byte, b'\n');
    } else {
        panic!("Expected UnprintableByte error");
    }
}

#[test]
fn test_new_reserved_byte() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+=/");
    if let Err(ParseAlphabetError::ReservedByte(byte)) = result {
        assert_eq!(byte, b'=');
    } else {
        panic!("Expected ReservedByte error");
    }
}

#[test]
fn test_new_duplicated_byte() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert!(result.is_ok()); // Base alphabet is okay
    let result_with_duplicate = Alphabet::new("ABCDEFGHIZJKLMNOPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz0123456789+/");
    if let Err(ParseAlphabetError::DuplicatedByte(byte)) = result_with_duplicate {
        assert_eq!(byte, b'I');
    } else {
        panic!("Expected DuplicatedByte error");
    }
}

