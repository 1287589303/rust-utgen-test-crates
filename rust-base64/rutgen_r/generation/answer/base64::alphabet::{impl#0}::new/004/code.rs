// Answer 0

#[test]
fn test_invalid_length() {
    let result = base64::Alphabet::new("short");
    assert!(result.is_err());
}

#[test]
fn test_unprintable_byte() {
    let result = base64::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+");
    assert!(result.is_err());
}

#[test]
fn test_reserved_byte() {
    let result = base64::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+");
    assert!(result.is_err());
}

#[test]
fn test_duplicate_byte() {
    let alphabet_with_duplicates = "ABCDDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // D is duplicated
    let result = base64::Alphabet::new(alphabet_with_duplicates);
    assert!(result.is_err());
}

