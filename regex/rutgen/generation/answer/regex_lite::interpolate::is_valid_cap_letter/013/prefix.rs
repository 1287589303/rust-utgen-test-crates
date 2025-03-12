// Answer 0

#[test]
fn test_is_valid_cap_letter_with_a() {
    let b: u8 = 97; // corresponds to 'a'
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_with_z() {
    let b: u8 = 122; // corresponds to 'z'
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_with_excluded_digit() {
    let b: u8 = 48; // corresponds to '0' (testing excluded from b'0'..=b'9')
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_with_excluded_high_digit() {
    let b: u8 = 57; // corresponds to '9' (testing excluded from b'0'..=b'9')
    is_valid_cap_letter(b);
}

