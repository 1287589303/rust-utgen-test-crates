// Answer 0

#[test]
fn test_new_alphabet_invalid_length() {
    let result = Alphabet::new("Short string");
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_alphabet_unprintable_byte() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x19");
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(0x19)));
}

#[test]
fn test_new_alphabet_reserved_byte() {
    const PAD_BYTE: u8 = b'='; // Assuming PAD_BYTE is defined as the byte for padding
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=");
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(PAD_BYTE)));
}

#[test]
fn test_new_alphabet_duplicated_byte() {
    let result = Alphabet::new("ABCDEFGHIJKLABCDEFGHIJKLmnopqrstuvwx");
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

