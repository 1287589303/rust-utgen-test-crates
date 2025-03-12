// Answer 0

#[test]
fn test_cursor_is_empty_with_non_empty_string() {
    let cursor = Cursor { rest: "test" };
    let _ = cursor.is_empty();
}

#[test]
fn test_cursor_is_empty_with_empty_string() {
    let cursor = Cursor { rest: "" };
    let _ = cursor.is_empty();
}

#[test]
fn test_cursor_is_empty_with_single_character_string() {
    let cursor = Cursor { rest: "a" };
    let _ = cursor.is_empty();
}

#[test]
fn test_cursor_is_empty_with_long_string() {
    let cursor = Cursor { rest: "a very long string" };
    let _ = cursor.is_empty();
}

#[test]
fn test_cursor_is_empty_with_whitespace_only_string() {
    let cursor = Cursor { rest: "   " };
    let _ = cursor.is_empty();
}

