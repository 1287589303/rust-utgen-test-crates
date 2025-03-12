// Answer 0

#[test]
fn test_is_word_byte_not_underscore() {
    let c: u8 = b'/'; // not matching b'_'
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_not_digit() {
    let c: u8 = b'!'; // not in b'0'..=b'9'
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_not_lowercase() {
    let c: u8 = b'`'; // not in b'a'..=b'z'
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_not_uppercase() {
    let c: u8 = b'{'; // not in b'A'..=b'Z'
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_not_special() {
    let c: u8 = b'~'; // not matching any in the specified ranges
    is_word_byte(c);
}

