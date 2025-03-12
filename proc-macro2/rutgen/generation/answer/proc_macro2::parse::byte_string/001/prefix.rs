// Answer 0

#[test]
fn test_byte_string_cooked() {
    let input = Cursor { rest: "b\"hello world\"" };
    let result = byte_string(input);
}

#[test]
fn test_byte_string_raw() {
    let input = Cursor { rest: "br\"hello world\"" };
    let result = byte_string(input);
}

#[test]
fn test_byte_string_invalid_start() {
    let input = Cursor { rest: "x\"invalid input\"" };
    let result = byte_string(input);
}

