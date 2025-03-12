// Answer 0

#[test]
fn test_is_passthrough_ascii_label_length_less_than_4() {
    let label: &[u8] = b"ab"; // Length is less than 4
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_valid_first_character_with_digit() {
    let label: &[u8] = b"a1bc"; // Valid first character, contains digit, valid length
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_invalid_character_in_tail() {
    let label: &[u8] = b"abc!"; // Invalid character '!' in tail, should fail
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_valid_last_character() {
    let label: &[u8] = b"a1b"; // Valid first character, doesn't end with '-', valid length
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_with_mixed_characters() {
    let label: &[u8] = b"a0b-c"; // Valid mix of characters, contains digit and valid last character
    let result = is_passthrough_ascii_label(label);
}

