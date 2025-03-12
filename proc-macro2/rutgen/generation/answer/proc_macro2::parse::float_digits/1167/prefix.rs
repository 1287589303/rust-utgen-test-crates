// Answer 0

#[test]
fn test_float_digits_error_non_digit_start() {
    let input = Cursor { rest: "a", off: 0 };
    let _ = float_digits(input);
}

#[test]
fn test_float_digits_error_invalid_character() {
    let input = Cursor { rest: "#10", off: 0 };
    let _ = float_digits(input);
}

#[test]
fn test_float_digits_error_invalid_float_no_digit() {
    let input = Cursor { rest: "e10", off: 0 };
    let _ = float_digits(input);
}

#[test]
fn test_float_digits_error_invalid_float_leading_zero() {
    let input = Cursor { rest: "0.1.2", off: 0 };
    let _ = float_digits(input);
}

#[test]
fn test_float_digits_error_invalid_float_ending_e() {
    let input = Cursor { rest: "12.34e", off: 0 };
    let _ = float_digits(input);
}

#[test]
fn test_float_digits_error_invalid_float_two_dots() {
    let input = Cursor { rest: "2..3", off: 0 };
    let _ = float_digits(input);
}

#[test]
fn test_float_digits_error_invalid_character_after_digit() {
    let input = Cursor { rest: "9a", off: 0 };
    let _ = float_digits(input);
}

