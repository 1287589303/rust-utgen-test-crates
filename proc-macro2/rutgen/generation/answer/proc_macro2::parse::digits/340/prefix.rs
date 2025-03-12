// Answer 0

#[test]
fn test_digits_decimal_123() {
    let cursor = Cursor { rest: "123" };
    let result = digits(cursor);
}

#[test]
fn test_digits_decimal_42_0() {
    let cursor = Cursor { rest: "42_0" };
    let result = digits(cursor);
}

#[test]
fn test_digits_decimal_1_2_3() {
    let cursor = Cursor { rest: "1_2_3" };
    let result = digits(cursor);
}

#[test]
fn test_digits_decimal_000() {
    let cursor = Cursor { rest: "000" };
    let result = digits(cursor);
}

#[test]
fn test_digits_decimal_with_underscore() {
    let cursor = Cursor { rest: "100_200" };
    let result = digits(cursor);
}

#[test]
fn test_digits_decimal_edge_case() {
    let cursor = Cursor { rest: "0" };
    let result = digits(cursor);
}

