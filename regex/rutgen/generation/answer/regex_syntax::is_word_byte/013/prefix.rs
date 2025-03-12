// Answer 0

#[test]
fn test_is_word_byte_underscore() {
    let c: u8 = b'_';
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_upper_a() {
    let c: u8 = b'A';
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_upper_z() {
    let c: u8 = b'Z';
    is_word_byte(c);
}

#[test]
#[should_panic]
fn test_is_word_byte_exclamation() {
    let c: u8 = b'!';
    is_word_byte(c);
}

#[test]
#[should_panic]
fn test_is_word_byte_at() {
    let c: u8 = b'@';
    is_word_byte(c);
}

#[test]
#[should_panic]
fn test_is_word_byte_lower_a_minus_1() {
    let c: u8 = b'a' - 1;
    is_word_byte(c);
}

#[test]
#[should_panic]
fn test_is_word_byte_nine_plus_1() {
    let c: u8 = b'9' + 1;
    is_word_byte(c);
}

