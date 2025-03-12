// Answer 0

#[test]
fn test_is_escapeable_character_uppercase_letter() {
    let c = 'A';
    is_escapeable_character(c);
}

#[test]
fn test_is_escapeable_character_lowercase_letter() {
    let c = 'a';
    is_escapeable_character(c);
}

#[test]
fn test_is_escapeable_character_digit() {
    let c = '3';
    is_escapeable_character(c);
}

