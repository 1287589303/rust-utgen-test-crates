// Answer 0

#[test]
fn test_invalid_length() {
    let result = base64::Alphabet::new("short");
    assert_eq!(result, Err(base64::ParseAlphabetError::InvalidLength));
}

#[test]
fn test_unprintable_byte() {
    let result = base64::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~\x07");
    assert_eq!(result, Err(base64::ParseAlphabetError::UnprintableByte(0x07)));
}

#[test]
fn test_reserved_byte_equals() {
    const PAD_BYTE: u8 = b'=';

    let mut alphabet = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+");
    alphabet.push(PAD_BYTE as char);

    let result = base64::Alphabet::new(&alphabet);
    assert_eq!(result, Err(base64::ParseAlphabetError::ReservedByte(PAD_BYTE)));
}

