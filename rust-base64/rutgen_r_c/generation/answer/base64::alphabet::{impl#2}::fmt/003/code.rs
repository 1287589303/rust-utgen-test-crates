// Answer 0

#[test]
fn test_parse_alphabet_error_display_invalid_length() {
    let error = ParseAlphabetError::InvalidLength;
    let result = format!("{}", error);
    assert_eq!(result, "Invalid length - must be 64 bytes");
}

#[test]
fn test_parse_alphabet_error_display_duplicated_byte() {
    let byte: u8 = 0xFF;
    let error = ParseAlphabetError::DuplicatedByte(byte);
    let result = format!("{}", error);
    assert_eq!(result, "Duplicated byte: 0xff");
}

#[test]
fn test_parse_alphabet_error_display_unprintable_byte() {
    let byte: u8 = 0x01; // example of an unprintable byte
    let error = ParseAlphabetError::UnprintableByte(byte);
    let result = format!("{}", error);
    assert_eq!(result, "Unprintable byte: 0x01");
}

#[test]
fn test_parse_alphabet_error_display_reserved_byte() {
    let byte: u8 = b'='; // example of a reserved byte
    let error = ParseAlphabetError::ReservedByte(byte);
    let result = format!("{}", error);
    assert_eq!(result, "Reserved byte: 0x3d");
}

