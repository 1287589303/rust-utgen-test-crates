// Answer 0

#[test]
fn test_is_escape_with_double_quote() {
    let ch: u8 = 34; // b'"'
    let including_control_characters: bool = true;
    let _ = is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_with_backslash() {
    let ch: u8 = 92; // b'\'
    let including_control_characters: bool = true;
    let _ = is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_with_control_character_0() {
    let ch: u8 = 0; // control character
    let including_control_characters: bool = true;
    let _ = is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_with_control_character_31() {
    let ch: u8 = 31; // control character
    let including_control_characters: bool = true;
    let _ = is_escape(ch, including_control_characters);
}

