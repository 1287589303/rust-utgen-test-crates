// Answer 0

#[test]
fn test_cooked_string_with_backslash() {
    let input_str = "\\";
    let cursor = Cursor { rest: input_str };
    let _ = cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_carriage_return() {
    let input_str = "\r";
    let cursor = Cursor { rest: input_str };
    let _ = cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_quotes() {
    let input_str = "\"";
    let cursor = Cursor { rest: input_str };
    let _ = cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_unsupported_character() {
    let input_str = "a";
    let cursor = Cursor { rest: input_str };
    let _ = cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_multiple_special_characters() {
    let input_str = "\\\r\"a";
    let cursor = Cursor { rest: input_str };
    let _ = cooked_string(cursor);
}

