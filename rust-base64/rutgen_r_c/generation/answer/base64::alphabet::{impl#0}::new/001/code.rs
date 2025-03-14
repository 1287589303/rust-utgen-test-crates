// Answer 0

#[test]
fn test_new_invalid_length_too_short() {
    let alphabet = "ABCDEF"; // Length 6, less than 64
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_invalid_length_too_long() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="; // Length 66, more than 64
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_invalid_length_exactly_64() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXZYabcdefghijklmnopqrstuvwxyz0123456789+/"; // Length 64, valid
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Ok(Alphabet::from_str_unchecked(alphabet)));
}

