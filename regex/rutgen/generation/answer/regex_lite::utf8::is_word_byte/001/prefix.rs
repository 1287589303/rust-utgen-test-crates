// Answer 0

#[test]
fn test_word_byte_underscore() {
    let input: u8 = 95;
    let result = is_word_byte(input);
}

#[test]
fn test_word_byte_digits() {
    for digit in 48..=57 {
        let result = is_word_byte(digit);
    }
}

#[test]
fn test_word_byte_uppercase() {
    for uppercase in 65..=90 {
        let result = is_word_byte(uppercase);
    }
}

#[test]
fn test_word_byte_lowercase() {
    for lowercase in 97..=122 {
        let result = is_word_byte(lowercase);
    }
}

#[test]
fn test_word_byte_outside_range() {
    let outside_bytes: [u8; 10] = [0, 32, 64, 91, 92, 93, 94, 96, 123, 255];
    for &byte in &outside_bytes {
        let result = is_word_byte(byte);
    }
}

