// Answer 0

#[test]
fn test_invalid_length() {
    use crate::ParseAlphabetError;

    let error = ParseAlphabetError::InvalidLength;
    let mut output = String::new();
    let result = error.fmt(&mut output);
}

#[test]
fn test_invalid_length_display() {
    use crate::ParseAlphabetError;

    let error = ParseAlphabetError::InvalidLength;
    let output = format!("{}", error);
}

