// Answer 0

#[test]
fn test_new_valid_alphabet() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert!(result.is_ok());
}

#[test]
fn test_new_invalid_length() {
    let result = Alphabet::new("ShortLength");
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_unprintable_byte() {
    let result = Alphabet::new("ABCDEF\x01GHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(1)));
}

#[test]
fn test_new_reserved_byte() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=");
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(61))); // 61 is the ASCII value for '='
}

#[test]
fn test_new_duplicated_byte() {
    let result = Alphabet::new("AAAABBBBCCCCDDDDEEEEFFFFFFFFGGGGHHHHIIIIJJJJKKKKLLLLMMMMNNNN");
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

