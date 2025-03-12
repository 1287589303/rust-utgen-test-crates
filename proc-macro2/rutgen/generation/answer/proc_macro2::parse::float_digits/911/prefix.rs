// Answer 0

#[test]
fn test_float_digits_with_exponential_and_positive_sign() {
    let cursor = Cursor { rest: "1.23e+45" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_exponential_and_negative_sign() {
    let cursor = Cursor { rest: "0.5E-12" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_with_exponential_and_no_sign() {
    let cursor = Cursor { rest: "12.34E56" };
    let _result = float_digits(cursor);
}

