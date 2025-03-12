// Answer 0

#[test]
fn test_is_escapeable_character_digit() {
    let input = '1';
    is_escapeable_character(input);
}

#[test]
fn test_is_escapeable_character_uppercase_letter() {
    let input = 'A';
    is_escapeable_character(input);
}

#[test]
fn test_is_escapeable_character_lowercase_letter() {
    let input = 'b';
    is_escapeable_character(input);
}

#[test]
fn test_is_escapeable_character_digit_boundary() {
    let input = '9';
    is_escapeable_character(input);
}

#[test]
fn test_is_escapeable_character_uppercase_letter_boundary() {
    let input = 'Z';
    is_escapeable_character(input);
}

#[test]
fn test_is_escapeable_character_lowercase_letter_boundary() {
    let input = 'a';
    is_escapeable_character(input);
}

