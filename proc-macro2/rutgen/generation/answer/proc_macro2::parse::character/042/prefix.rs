// Answer 0

#[test]
fn test_character_valid_escape_u() {
    let cursor = Cursor { rest: "'\\u123'".to_string().as_str() };
    let _ = character(cursor);
}

#[test]
fn test_character_valid_escape_x() {
    let cursor = Cursor { rest: "'\\x1a'".to_string().as_str() };
    let _ = character(cursor);
}

#[test]
fn test_character_valid_escape_newline() {
    let cursor = Cursor { rest: "'\\n'".to_string().as_str() };
    let _ = character(cursor);
}

#[test]
fn test_character_valid_escape_return() {
    let cursor = Cursor { rest: "'\\r'".to_string().as_str() };
    let _ = character(cursor);
}

#[test]
fn test_character_valid_escape_tab() {
    let cursor = Cursor { rest: "'\\t'".to_string().as_str() };
    let _ = character(cursor);
}

#[test]
fn test_character_valid_escape_backslash() {
    let cursor = Cursor { rest: "'\\\\'".to_string().as_str() };
    let _ = character(cursor);
}

#[test]
fn test_character_valid_escape_zero() {
    let cursor = Cursor { rest: "'\\0'".to_string().as_str() };
    let _ = character(cursor);
}

#[test]
fn test_character_valid_escape_single_quote() {
    let cursor = Cursor { rest: "'\\''".to_string().as_str() };
    let _ = character(cursor);
}

#[test]
fn test_character_valid_escape_double_quote() {
    let cursor = Cursor { rest: "'\"'".to_string().as_str() };
    let _ = character(cursor);
}

#[test]
fn test_character_invalid_no_closing_quote() {
    let cursor = Cursor { rest: "'\\n".to_string().as_str() };
    let _ = character(cursor);
}

#[test]
fn test_character_invalid_unrecognized_escape() {
    let cursor = Cursor { rest: "'\\z'".to_string().as_str() };
    let _ = character(cursor);
}

#[test]
fn test_character_invalid_empty_string() {
    let cursor = Cursor { rest: "''".to_string().as_str() };
    let _ = character(cursor);
}

