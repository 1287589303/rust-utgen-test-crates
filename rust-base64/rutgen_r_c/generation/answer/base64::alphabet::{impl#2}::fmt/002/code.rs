// Answer 0

#[test]
fn test_unprintable_byte_display() {
    use crate::ParseAlphabetError;

    let error = ParseAlphabetError::UnprintableByte(0x1F); // An unprintable byte
    let expected = "Unprintable byte: 0x1f";
    let result = format!("{}", error);

    assert_eq!(result, expected);
}

#[test]
fn test_invalid_length_display() {
    use crate::ParseAlphabetError;

    let error = ParseAlphabetError::InvalidLength; // Invalid length
    let expected = "Invalid length - must be 64 bytes";
    let result = format!("{}", error);

    assert_eq!(result, expected);
}

#[test]
fn test_duplicated_byte_display() {
    use crate::ParseAlphabetError;

    let error = ParseAlphabetError::DuplicatedByte(0x2A); // A duplicated byte example
    let expected = "Duplicated byte: 0x2a";
    let result = format!("{}", error);

    assert_eq!(result, expected);
}

#[test]
fn test_reserved_byte_display() {
    use crate::ParseAlphabetError;

    let error = ParseAlphabetError::ReservedByte(0x3D); // Reserved byte '='
    let expected = "Reserved byte: 0x3d";
    let result = format!("{}", error);

    assert_eq!(result, expected);
}

