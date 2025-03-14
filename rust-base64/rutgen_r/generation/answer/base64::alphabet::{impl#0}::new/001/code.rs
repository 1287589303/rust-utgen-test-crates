// Answer 0

#[test]
fn test_new_alphabet_invalid_length_too_short() {
    let alphabet = "ABCDEFGHIGKLMNOPQRSTUVWXYZ"; // 62 characters, should be 64
    let result = base64::Alphabet::new(alphabet);
    assert_eq!(result, Err(base64::ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_alphabet_invalid_length_too_long() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"; // 52 characters, should be 64
    let result = base64::Alphabet::new(alphabet);
    assert_eq!(result, Err(base64::ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_alphabet_invalid_length_empty() {
    let alphabet = ""; // 0 characters, should be 64
    let result = base64::Alphabet::new(alphabet);
    assert_eq!(result, Err(base64::ParseAlphabetError::InvalidLength));
}

