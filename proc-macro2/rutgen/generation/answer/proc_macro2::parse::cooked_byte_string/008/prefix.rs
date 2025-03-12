// Answer 0

#[test]
fn test_cooked_byte_string_with_trailing_backslash_escape_sequence() {
    let cursor = Cursor { rest: "\\n".to_string().as_str() };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_trailing_backslash_and_newline() {
    let cursor = Cursor { rest: "\\r\n".to_string().as_str() };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_multiple_backslashes() {
    let cursor = Cursor { rest: "\\\\x".to_string().as_str() };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_valid_escape() {
    let cursor = Cursor { rest: "\\t".to_string().as_str() };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_ending_backslash() {
    let cursor = Cursor { rest: "\\\\".to_string().as_str() };
    let _ = cooked_byte_string(cursor);
}

