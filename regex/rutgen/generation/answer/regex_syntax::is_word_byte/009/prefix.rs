// Answer 0

#[test]
fn test_is_word_byte_underscore() {
    let input = b'_';
    is_word_byte(input);
}

#[test]
fn test_is_word_byte_lowercase_a() {
    let input = b'a';
    is_word_byte(input);
}

#[test]
fn test_is_word_byte_lowercase_z() {
    let input = b'z';
    is_word_byte(input);
}

#[test]
fn test_is_word_byte_uppercase_a() {
    let input = b'A';
    is_word_byte(input);
}

#[test]
fn test_is_word_byte_uppercase_z() {
    let input = b'Z';
    is_word_byte(input);
}

