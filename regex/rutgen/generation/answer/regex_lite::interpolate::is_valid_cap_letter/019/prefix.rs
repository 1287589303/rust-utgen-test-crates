// Answer 0

#[test]
fn test_is_valid_cap_letter_uppercase_A() {
    let input = b'A';
    is_valid_cap_letter(input);
}

#[test]
fn test_is_valid_cap_letter_uppercase_Z() {
    let input = b'Z';
    is_valid_cap_letter(input);
}

#[test]
fn test_is_valid_cap_letter_underscore() {
    let input = b'_';
    is_valid_cap_letter(input);
}

