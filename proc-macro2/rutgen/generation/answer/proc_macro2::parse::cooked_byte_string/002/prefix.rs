// Answer 0

#[test]
fn test_cooked_byte_string_with_double_quote() {
    let cursor = Cursor { rest: r#"abc"def"# };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_carriage_return() {
    let cursor = Cursor { rest: "abc\rdef" };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_backslash() {
    let cursor = Cursor { rest: "abc\\def" };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_invalid_sequence() {
    let cursor = Cursor { rest: "abc\\x10def" };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_ascii() {
    let cursor = Cursor { rest: "abc def" };
    let _ = cooked_byte_string(cursor);
}

