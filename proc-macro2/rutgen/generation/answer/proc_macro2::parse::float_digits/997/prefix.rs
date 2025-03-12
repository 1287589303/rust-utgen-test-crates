// Answer 0

#[test]
fn test_float_digits_with_single_digit_no_dot() {
    let cursor = Cursor { rest: "5" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_multiple_digits_no_dot() {
    let cursor = Cursor { rest: "789" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_single_digit_and_dot() {
    let cursor = Cursor { rest: "1.23" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_multiple_digits_and_dot() {
    let cursor = Cursor { rest: "4.5" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_exponent_part() {
    let cursor = Cursor { rest: "123E+1" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_single_digit_no_dot_and_exponent() {
    let cursor = Cursor { rest: "9E-2" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_dot_and_exponent() {
    let cursor = Cursor { rest: "3.14E-3" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_multiple_underscores() {
    let cursor = Cursor { rest: "1_000.23E+1" };
    let result = float_digits(cursor);
}

