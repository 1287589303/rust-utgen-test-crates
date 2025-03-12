// Answer 0

#[test]
fn test_count_digits_in_range_10_to_99() {
    let input_value: u16 = 10;
    let result = count_digits(input_value);
}

#[test]
fn test_count_digits_in_range_10_to_99_another_example() {
    let input_value: u16 = 55;
    let result = count_digits(input_value);
}

#[test]
fn test_count_digits_in_range_10_to_99_boundary() {
    let input_value: u16 = 99;
    let result = count_digits(input_value);
}

