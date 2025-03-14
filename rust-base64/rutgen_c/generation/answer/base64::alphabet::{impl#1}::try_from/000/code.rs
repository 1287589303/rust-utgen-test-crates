// Answer 0

#[test]
fn test_try_from_valid_alphabet() {
    let result = Alphabet::try_from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_try_from_invalid_length() {
    let result = Alphabet::try_from("short");
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_try_from_unprintable_byte() {
    let result = Alphabet::try_from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0");
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(0)));
}

#[test]
fn test_try_from_reserved_byte() {
    let result = Alphabet::try_from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+=");
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(b'=')));
}

#[test]
fn test_try_from_duplicated_bytes() {
    let result = Alphabet::try_from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/A");
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

