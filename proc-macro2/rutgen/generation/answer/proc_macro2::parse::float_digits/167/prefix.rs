// Answer 0

#[test]
fn test_float_digits_basic_zero() {
    let cursor = Cursor { rest: "0" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_single_digit() {
    let cursor = Cursor { rest: "4" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_with_underscore() {
    let cursor = Cursor { rest: "5_6" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_with_multiple_digits() {
    let cursor = Cursor { rest: "123456" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_with_invalid_characters() {
    let cursor = Cursor { rest: "789abc" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_without_dot_or_exponent() {
    let cursor = Cursor { rest: "00_0_7" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_single_underscore() {
    let cursor = Cursor { rest: "3_6" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_invalid_starting_char() {
    let cursor = Cursor { rest: "8.a" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_exponent_without_value() {
    let cursor = Cursor { rest: "2e" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_exponent_with_invalid_sign() {
    let cursor = Cursor { rest: "9e+" };
    let _ = float_digits(cursor);
}

