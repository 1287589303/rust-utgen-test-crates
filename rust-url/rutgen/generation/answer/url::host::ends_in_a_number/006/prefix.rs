// Answer 0

#[test]
fn test_ends_in_a_number_with_digit_segment() {
    let input = "example.123";
    ends_in_a_number(input);
}

#[test]
fn test_ends_in_a_number_with_multiple_digits() {
    let input = "test.4567";
    ends_in_a_number(input);
}

#[test]
fn test_ends_in_a_number_with_zero() {
    let input = "number.0";
    ends_in_a_number(input);
}

#[test]
fn test_ends_in_a_number_with_large_digit_segment() {
    let input = "data.987654321";
    ends_in_a_number(input);
}

