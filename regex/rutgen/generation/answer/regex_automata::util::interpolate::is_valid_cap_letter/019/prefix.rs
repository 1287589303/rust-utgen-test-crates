// Answer 0

#[test]
fn test_is_valid_cap_letter_uppercase_A() {
    let input: u8 = b'A';
    is_valid_cap_letter(input);
}

#[test]
fn test_is_valid_cap_letter_uppercase_Z() {
    let input: u8 = b'Z';
    is_valid_cap_letter(input);
}

#[test]
fn test_is_valid_cap_letter_underscore() {
    let input: u8 = b'_';
    is_valid_cap_letter(input);
}

