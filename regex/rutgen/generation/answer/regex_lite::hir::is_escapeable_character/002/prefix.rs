// Answer 0

#[test]
fn test_is_escapeable_character_digit() {
    let result = is_escapeable_character('0');
}

#[test]
fn test_is_escapeable_character_digit_upper() {
    let result = is_escapeable_character('5');
}

#[test]
fn test_is_escapeable_character_digit_upper_case() {
    let result = is_escapeable_character('9');
}

#[test]
fn test_is_escapeable_character_lower_case() {
    let result = is_escapeable_character('a');
}

#[test]
fn test_is_escapeable_character_upper_case() {
    let result = is_escapeable_character('Z');
}

#[test]
fn test_is_escapeable_character_lower_case_b() {
    let result = is_escapeable_character('b');
}

#[test]
fn test_is_escapeable_character_special_character() {
    let result = is_escapeable_character('@');
}

