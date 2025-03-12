// Answer 0

#[test]
fn test_byte_with_valid_escape_n() {
    let input = Cursor { rest: "b'\\n'" };
    let result = byte(input);
}

#[test]
fn test_byte_with_valid_escape_r() {
    let input = Cursor { rest: "b'\\r'" };
    let result = byte(input);
}

#[test]
fn test_byte_with_valid_escape_t() {
    let input = Cursor { rest: "b'\\t'" };
    let result = byte(input);
}

#[test]
fn test_byte_with_valid_escape_backslash() {
    let input = Cursor { rest: "b'\\\\'" };
    let result = byte(input);
}

#[test]
fn test_byte_with_valid_escape_0() {
    let input = Cursor { rest: "b'\\0'" };
    let result = byte(input);
}

#[test]
fn test_byte_with_valid_escape_single_quote() {
    let input = Cursor { rest: "b'\\''" };
    let result = byte(input);
}

#[test]
fn test_byte_with_valid_escape_double_quote() {
    let input = Cursor { rest: "b'\"'" };
    let result = byte(input);
}

#[test]
fn test_byte_with_valid_hex_escape() {
    let input = Cursor { rest: "b'\\x0A'" };
    let result = byte(input);
}

