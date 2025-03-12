// Answer 0

#[test]
fn test_digits_with_binary_and_invalid_hex_digit() {
    let input = Cursor { rest: "0bA3" };
    let _result = digits(input);
}

#[test]
fn test_digits_with_binary_and_invalid_hex_digit_with_underscore() {
    let input = Cursor { rest: "0bB_" };
    let _result = digits(input);
}

#[test]
fn test_digits_with_binary_and_hex_digit_exceeding_base() {
    let input = Cursor { rest: "0bF" };
    let _result = digits(input);
}

