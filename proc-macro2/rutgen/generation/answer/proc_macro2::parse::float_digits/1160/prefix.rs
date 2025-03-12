// Answer 0

#[test]
fn test_float_digits_case_with_exp_and_underscore() {
    let cursor = Cursor { rest: "1_2E" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_case_with_exp_and_sign() {
    let cursor = Cursor { rest: "1_2E+" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_case_with_exp_and_dot() {
    let cursor = Cursor { rest: "1.2E+" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_case_with_exp_negative_sign() {
    let cursor = Cursor { rest: "1_2E-" };
    let _result = float_digits(cursor);
}

#[test]
fn test_float_digits_case_with_exp_multiple_underscores() {
    let cursor = Cursor { rest: "1__2E" };
    let _result = float_digits(cursor);
}

