// Answer 0

#[test]
fn test_literal_nocapture_string() {
    let cursor = Cursor { rest: "\"valid string\" and some other text" };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_byte_string() {
    let cursor = Cursor { rest: "b\"valid byte string\" and more text" };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_c_string() {
    let cursor = Cursor { rest: "c\"valid C string\" and additional text" };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_byte_character() {
    let cursor = Cursor { rest: "b'c' and other data" };
    let _ = literal_nocapture(cursor);
}

#[test]
fn test_literal_nocapture_invalid_float() {
    let cursor = Cursor { rest: "123.45.67 and more text" };
    let _ = literal_nocapture(cursor);
}

