// Answer 0

#[test]
fn test_float_digits_with_valid_input() {
    let cursor = Cursor { rest: "0_._1e+2" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_invalid_initial_digit() {
    let cursor = Cursor { rest: "0_1e+2" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_valid_input_and_exponent() {
    let cursor = Cursor { rest: "0_1e-2" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_invalid_input_for_exponent() {
    let cursor = Cursor { rest: "0_1e+_" };
    let result = float_digits(cursor);
}

