// Answer 0

#[test]
fn test_fmt_reserved_byte() {
    let reserved_byte = ParseAlphabetError::ReservedByte(0x3d); // Example reserved byte '='
    let mut output = String::new();
    let result = write!(&mut output, "{}", reserved_byte);
    assert!(result.is_ok());
    assert_eq!(output, "Reserved byte: 0x3d");
}

#[test]
fn test_fmt_invalid_length() {
    let invalid_length = ParseAlphabetError::InvalidLength;
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_length);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid length - must be 64 bytes");
}

#[test]
fn test_fmt_duplicated_byte() {
    let duplicated_byte = ParseAlphabetError::DuplicatedByte(0x20); // Example duplicated byte ' '
    let mut output = String::new();
    let result = write!(&mut output, "{}", duplicated_byte);
    assert!(result.is_ok());
    assert_eq!(output, "Duplicated byte: 0x20");
}

#[test]
fn test_fmt_unprintable_byte() {
    let unprintable_byte = ParseAlphabetError::UnprintableByte(0x1f); // Example unprintable byte
    let mut output = String::new();
    let result = write!(&mut output, "{}", unprintable_byte);
    assert!(result.is_ok());
    assert_eq!(output, "Unprintable byte: 0x1f");
}

