// Answer 0

#[test]
fn test_value_to_digit_a() {
    let value: u32 = 0;
    let result = value_to_digit(value);
}

#[test]
fn test_value_to_digit_b() {
    let value: u32 = 1;
    let result = value_to_digit(value);
}

#[test]
fn test_value_to_digit_z() {
    let value: u32 = 25;
    let result = value_to_digit(value);
}

