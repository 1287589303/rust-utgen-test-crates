// Answer 0

#[test]
fn test_cooked_byte_string_non_ascii_characters() {
    let input_str = "valid\\string\r\nwith\0nonascii";
    let cursor = Cursor { rest: input_str };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_double_quote() {
    let input_str = "valid\\string\"";
    let cursor = Cursor { rest: input_str };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_carriage_return() {
    let input_str = "valid\\string\r";
    let cursor = Cursor { rest: input_str };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_backslash() {
    let input_str = "valid\\string\\";
    let cursor = Cursor { rest: input_str };
    let _ = cooked_byte_string(cursor);
}

#[test]
fn test_cooked_byte_string_with_invalid_character() {
    let input_str = "valid\\string\xFF";
    let cursor = Cursor { rest: input_str };
    let _ = cooked_byte_string(cursor);
}

