// Answer 0

#[test]
fn test_digits_valid_decimal() {
    let input = Cursor { rest: "123" };
    let result = digits(input);
}

#[test]
fn test_digits_valid_decimal_with_leading_digit() {
    let input = Cursor { rest: "42" };
    let result = digits(input);
}

#[test]
fn test_digits_valid_decimal_endless_numbers() {
    let input = Cursor { rest: "100" };
    let result = digits(input);
}

