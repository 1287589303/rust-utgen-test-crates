// Answer 0

#[test]
fn test_is_valid_cap_letter_digit_0() {
    let b = 48; // ASCII for '0'
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_digit_9() {
    let b = 57; // ASCII for '9'
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_uppercase_A() {
    let b = 65; // ASCII for 'A'
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_uppercase_Z() {
    let b = 90; // ASCII for 'Z'
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_lowercase_a() {
    let b = 97; // ASCII for 'a'
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_lowercase_z() {
    let b = 122; // ASCII for 'z'
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_underscore() {
    let b = 95; // ASCII for '_'
    is_valid_cap_letter(b);
}

