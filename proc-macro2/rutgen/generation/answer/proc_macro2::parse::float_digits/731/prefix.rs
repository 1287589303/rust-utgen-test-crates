// Answer 0

#[test]
fn test_float_digits_valid_float_with_exponent() {
    let cursor = Cursor { rest: "3.5E+2" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_valid_float_with_exponent_negative() {
    let cursor = Cursor { rest: "4.0e-1" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_valid_float_with_underscore() {
    let cursor = Cursor { rest: "2_3.0_1E+4" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_invalid_multiple_dots() {
    let cursor = Cursor { rest: "5..3E1" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_invalid_exponent_no_digits() {
    let cursor = Cursor { rest: "1.5E" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_invalid_exponent_sign_without_digits() {
    let cursor = Cursor { rest: "6.3E+" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_invalid_exponent_multiple_signs() {
    let cursor = Cursor { rest: "9.8E++1" };
    let _result = float_digits(cursor);
}

