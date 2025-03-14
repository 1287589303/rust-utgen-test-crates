// Answer 0

#[test]
fn test_new_invalid_length() {
    let result = Alphabet::new("short");
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_unprintable_byte() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0");
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(0)));
}

#[test]
fn test_new_reserved_byte() {
    const PAD_BYTE: u8 = b'='; // Assume PAD_BYTE is '='
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=");
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(PAD_BYTE)));
}

#[test]
fn test_new_duplicated_byte() {
    let result = Alphabet::new("ABCDABCDEFABCDEFGHIJKLMOPQRSTUVWXYZ0123456789+/");
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

#[test]
fn test_new_exceed_upper_bound() {
    // Test upper boundary where all bytes are valid and unique
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/-");
    assert!(result.is_ok());
}

