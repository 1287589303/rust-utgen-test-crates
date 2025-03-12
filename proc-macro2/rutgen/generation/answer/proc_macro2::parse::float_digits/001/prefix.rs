// Answer 0

#[test]
fn test_float_digits_empty_input() {
    let cursor = Cursor { rest: "" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_single_non_digit_input() {
    let cursor = Cursor { rest: "a" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_invalid_characters() {
    let cursor = Cursor { rest: "!" };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_whitespace_input() {
    let cursor = Cursor { rest: " " };
    let result = float_digits(cursor);
}

#[test]
fn test_float_digits_special_characters() {
    let cursor = Cursor { rest: "%^&*" };
    let result = float_digits(cursor);
}

