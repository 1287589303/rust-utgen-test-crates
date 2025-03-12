// Answer 0

#[test]
fn test_cooked_string_with_valid_escape_u() {
    let input = Cursor { rest: "\\u0061test" };
    let result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_valid_escape_x() {
    let input = Cursor { rest: "\\x61test" };
    let result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_invalid_escape_u() {
    let input = Cursor { rest: "\\uZZZZ" };
    let result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_trailing_newline() {
    let input = Cursor { rest: "\\\n" };
    let result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_valid_escape_sequence() {
    let input = Cursor { rest: "\\n" };
    let result = cooked_string(input);
}

