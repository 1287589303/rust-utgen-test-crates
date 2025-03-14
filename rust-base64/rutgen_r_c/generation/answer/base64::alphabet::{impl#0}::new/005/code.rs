// Answer 0

#[test]
fn test_new_alphabet_valid() {
    const VALID_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let result = Alphabet::new(VALID_ALPHABET);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_new_alphabet_invalid_length() {
    const INVALID_ALPHABET: &str = "ABC"; // Length is not 64
    
    let result = Alphabet::new(INVALID_ALPHABET);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_alphabet_unprintable_byte() {
    const INVALID_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x00"; // Non-printable byte
    
    let result = Alphabet::new(INVALID_ALPHABET);
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(b'\x00')));
}

#[test]
fn test_new_alphabet_reserved_byte() {
    const INVALID_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="; // '=' is a reserved byte
    
    let result = Alphabet::new(INVALID_ALPHABET);
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(b'=')));
}

#[test]
fn test_new_alphabet_duplicated_byte() {
    const INVALID_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x2B"; // Duplicate '+'
    
    let result = Alphabet::new(INVALID_ALPHABET);
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'+')));
}

