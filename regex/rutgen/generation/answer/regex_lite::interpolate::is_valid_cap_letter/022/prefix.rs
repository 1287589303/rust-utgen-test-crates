// Answer 0

#[test]
fn test_is_valid_cap_letter_invalid_non_alphanumeric_1() {
    let input = b'!';
    is_valid_cap_letter(input);
}

#[test]
fn test_is_valid_cap_letter_invalid_non_alphanumeric_2() {
    let input = b'@';
    is_valid_cap_letter(input);
}

#[test]
fn test_is_valid_cap_letter_invalid_non_alphanumeric_3() {
    let input = b'#';
    is_valid_cap_letter(input);
}

#[test]
fn test_is_valid_cap_letter_invalid_non_alphanumeric_4() {
    let input = b'$';
    is_valid_cap_letter(input);
}

#[test]
fn test_is_valid_cap_letter_invalid_non_alphanumeric_5() {
    let input = b'%';
    is_valid_cap_letter(input);
}

