// Answer 0

#[test]
fn test_alphabet_length_too_short() {
    let result = Alphabet::new("ABCDEF"); // 6 characters
}

#[test]
fn test_alphabet_length_too_long() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/123"); // 70 characters
}

#[test]
fn test_alphabet_length_empty() {
    let result = Alphabet::new(""); // 0 characters
}

#[test]
fn test_alphabet_length_maximum() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/1234567890abcdefg"); // 100 characters
}

