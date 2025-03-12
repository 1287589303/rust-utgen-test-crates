// Answer 0

#[test]
fn test_is_escapeable_character_non_ascii_character() {
    let result = is_escapeable_character('☃');
}

#[test]
fn test_is_escapeable_character_unicode_character() {
    let result = is_escapeable_character('😊');
}

#[test]
fn test_is_escapeable_character_non_ascii_special_character() {
    let result = is_escapeable_character('中');
}

#[test]
fn test_is_escapeable_character_non_ascii_symbol() {
    let result = is_escapeable_character('ß');
}

