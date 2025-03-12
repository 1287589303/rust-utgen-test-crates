// Answer 0

#[test]
fn test_is_escape_double_quote_without_control() {
    let ch: u8 = 34; // b'"'
    let including_control_characters: bool = false;
    is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_backslash_without_control() {
    let ch: u8 = 92; // b'\'
    let including_control_characters: bool = false;
    is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_control_character_with_control() {
    let ch: u8 = 0; // b'\0'
    let including_control_characters: bool = true;
    is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_non_control_character_with_control() {
    let ch: u8 = 32; // space
    let including_control_characters: bool = true;
    is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_beyond_control_with_control() {
    let ch: u8 = 31; // just below control character range
    let including_control_characters: bool = true;
    is_escape(ch, including_control_characters);
}

