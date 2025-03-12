// Answer 0

#[test]
fn test_digits_with_single_underscore() {
    let input = Cursor { rest: "_" };
    let result = digits(input);
}

#[test]
fn test_digits_with_multiple_underscores() {
    let input = Cursor { rest: "__" };
    let result = digits(input);
}

#[test]
fn test_digits_with_leading_underscore() {
    let input = Cursor { rest: "_123" };
    let result = digits(input);
}

#[test]
fn test_digits_with_only_underscores() {
    let input = Cursor { rest: "____" };
    let result = digits(input);
}

#[test]
fn test_digits_with_underscore_followed_by_invalid_char() {
    let input = Cursor { rest: "_abc" };
    let result = digits(input);
}

