// Answer 0

#[test]
fn test_cooked_c_string_with_simple_string() {
    let cursor = Cursor { rest: r#""Hello, World!""# };
    let result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_escaped_quotes() {
    let cursor = Cursor { rest: r#""Say \"Hello\" to everyone.""# };
    let result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_newline_inside() {
    let cursor = Cursor { rest: r#""First Line\nSecond Line""# };
    let result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_carriage_return_newline() {
    let cursor = Cursor { rest: r#""First Line\r\nSecond Line""# };
    let result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_leading_spaces() {
    let cursor = Cursor { rest: r#""   Leading spaces""# };
    let result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_trailing_spaces() {
    let cursor = Cursor { rest: r#""Trailing spaces   ""# };
    let result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_backslash() {
    let cursor = Cursor { rest: r#""This is a backslash: \\"# };
    let result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_multiple_escape_sequences() {
    let cursor = Cursor { rest: r#""Tab character:\tNew line:\nBackslash:\\\"# };
    let result = cooked_c_string(cursor);
}

