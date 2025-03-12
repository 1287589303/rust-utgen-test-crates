// Answer 0

#[test]
fn test_ends_in_a_number_last_is_empty_true_with_valid_previous_segment() {
    let input = "192.168.1.a";
    ends_in_a_number(input);
}

#[test]
fn test_ends_in_a_number_last_is_empty_true_with_octal_previous_segment() {
    let input = "0o75.0.a";
    ends_in_a_number(input);
}

#[test]
fn test_ends_in_a_number_last_is_empty_true_with_hexadecimal_previous_segment() {
    let input = "0x1A.0.b";
    ends_in_a_number(input);
}

#[test]
fn test_ends_in_a_number_last_is_empty_true_with_mixed_segments() {
    let input = "172.16.30.abc";
    ends_in_a_number(input);
}

#[test]
fn test_ends_in_a_number_last_is_empty_true_with_mixed_symbols() {
    let input = "255.255.255.a@#!";
    ends_in_a_number(input);
}

