// Answer 0

#[test]
fn test_byte_string_with_rest_starts_with_br() {
    let input = Cursor { rest: "brabc" };
    let _ = byte_string(input);
}

#[test]
fn test_byte_string_with_non_ascii_character() {
    let input = Cursor { rest: "b\"©" };
    let _ = byte_string(input);
}

#[test]
fn test_byte_string_with_empty_rest() {
    let input = Cursor { rest: "" };
    let _ = byte_string(input);
}

