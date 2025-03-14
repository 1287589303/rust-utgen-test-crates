// Answer 0

#[test]
fn test_alphabet_creation_valid() {
    let alphabet_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // Valid alphabet
    let result = Alphabet::new(alphabet_str);
    assert!(result.is_ok());
}

#[test]
fn test_alphabet_creation_invalid_length() {
    let alphabet_str = "ABCD"; // Invalid length
    let result = Alphabet::new(alphabet_str);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_alphabet_creation_unprintable_byte() {
    let alphabet_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x01"; // Contains unprintable byte
    let result = Alphabet::new(alphabet_str);
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(1)));
}

#[test]
fn test_alphabet_creation_reserved_byte() {
    let alphabet_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+="; // Contains reserved byte '='
    let result = Alphabet::new(alphabet_str);
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(b'=')));
}

#[test]
fn test_alphabet_creation_duplicated_byte() {
    let alphabet_str = "ABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEF"; // Contains duplicated byte 'A'
    let result = Alphabet::new(alphabet_str);
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

#[test]
fn test_alphabet_creation_boundary_conditions() {
    let alphabet_str_low = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]_^"; // lowest byte 32
    let result_low = Alphabet::new(alphabet_str_low);
    assert!(result_low.is_ok());

    let alphabet_str_high = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result_high = Alphabet::new(alphabet_str_high);
    assert!(result_high.is_ok());
}

