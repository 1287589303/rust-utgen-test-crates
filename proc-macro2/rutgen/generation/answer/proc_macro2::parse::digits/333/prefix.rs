// Answer 0

#[test]
fn test_digits_with_underscore() {
    let input = Cursor { rest: "12_34a" };
    let result = digits(input);
}

#[test]
fn test_digits_with_multiple_underscores() {
    let input = Cursor { rest: "1_2_3_" };
    let result = digits(input);
}

#[test]
fn test_digits_with_underscore_at_end() {
    let input = Cursor { rest: "9_" };
    let result = digits(input);
}

#[test]
fn test_digits_with_leading_underscore() {
    let input = Cursor { rest: "_23a" };
    let result = digits(input);
}

#[test]
fn test_digits_with_non_digit_after_underscore() {
    let input = Cursor { rest: "123_abc" };
    let result = digits(input);
}

