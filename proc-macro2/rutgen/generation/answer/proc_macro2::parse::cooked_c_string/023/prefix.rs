// Answer 0

#[test]
fn test_cooked_c_string_with_carriage_return_followed_by_newline() {
    let cursor = Cursor { rest: "\r\n" };
    let result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_carriage_return_followed_by_newline_and_no_extra_characters() {
    let cursor = Cursor { rest: "\r\n " };
    let result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_only_carriage_return_followed_by_newline() {
    let cursor = Cursor { rest: "\r\n" };
    let result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_multiple_carriage_returns_followed_by_newline() {
    let cursor = Cursor { rest: "\r\r\n" };
    let result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_trailing_whitespace_after_carriage_return_newline() {
    let cursor = Cursor { rest: "\r\n    " };
    let result = cooked_c_string(cursor);
}

