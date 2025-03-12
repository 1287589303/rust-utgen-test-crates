// Answer 0

#[test]
fn test_digits_with_hexadecimal_uppercase() {
    let input = Cursor { rest: "AFA", off: 0 };
    let result = digits(input);
}

#[test]
fn test_digits_with_single_hexadecimal_uppercase() {
    let input = Cursor { rest: "B", off: 0 };
    let result = digits(input);
}

#[test]
fn test_digits_with_hexadecimal_uppercase_and_underscore() {
    let input = Cursor { rest: "C_D_E", off: 0 };
    let result = digits(input);
}

#[test]
fn test_digits_with_hexadecimal_uppercase_longer_string() {
    let input = Cursor { rest: "F0F1F2F3F4F5F6F7", off: 0 };
    let result = digits(input);
}

#[test]
fn test_digits_with_hexadecimal_uppercase_empty_after() {
    let input = Cursor { rest: "AA", off: 0 };
    let result = digits(input);
}

