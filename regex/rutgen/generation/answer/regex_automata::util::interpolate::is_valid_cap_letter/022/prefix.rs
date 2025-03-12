// Answer 0

#[test]
fn test_is_valid_cap_letter_underflow() {
    let b: u8 = b'!'; // Outside the valid ranges
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_below_lowercase_a() {
    let b: u8 = b'`'; // Lower than 'a'
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_above_uppercase_z() {
    let b: u8 = b'['; // Higher than 'Z'
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_non_alpha_numeric() {
    let b: u8 = b'@'; // Non-alphanumeric character not valid
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_underscore() {
    let b: u8 = b'_'; // Valid character
    is_valid_cap_letter(b);
}

