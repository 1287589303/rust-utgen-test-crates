// Answer 0

#[test]
fn test_try_from_valid_alphabet() {
    let alphabet_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::try_from(alphabet_str);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), alphabet_str);
}

#[test]
fn test_try_from_invalid_length() {
    let alphabet_str = "short";
    let result = Alphabet::try_from(alphabet_str);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_try_from_unprintable_byte() {
    let alphabet_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x00";
    let result = Alphabet::try_from(alphabet_str);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(ParseAlphabetError::UnprintableByte(0)));
}

#[test]
fn test_try_from_reserved_byte() {
    let alphabet_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+=/";
    let result = Alphabet::try_from(alphabet_str);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(ParseAlphabetError::ReservedByte(b'=')));
}

#[test]
fn test_try_from_duplicated_byte() {
    let alphabet_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+ABCDEFGHIJKLMN";
    let result = Alphabet::try_from(alphabet_str);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(ParseAlphabetError::DuplicatedByte(b'A')));
}

